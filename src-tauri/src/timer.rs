use std::{process::exit, sync::{Arc, Condvar, Mutex}, thread::{self, sleep, JoinHandle}, time::{Duration, Instant}};

use tauri::{AppHandle, Emitter};


// #[derive(Clone, Copy, PartialEq, Debug)]
// #[derive(serde::Serialize)]
// pub enum TimerState {
//     Paused,
//     Running,
// }

pub struct Timer {
    // state: Arc<Mutex<TimerState>>,
    elapsed: Arc<Mutex<Duration>>,
    total: Arc<Mutex<Duration>>,
    paused_condvar: Arc<(Mutex<bool>,Condvar)>,
    tick_complete_condvar: Arc<(Mutex<bool>,Condvar)>,

    handle: Option<JoinHandle<()>>,
}

impl Timer {
    pub fn new(app: AppHandle) -> Self{

        // let state = Arc::new(Mutex::new(TimerState::Paused));
        let elapsed = Arc::new(Mutex::new(Duration::ZERO));
        let total = Arc::new(Mutex::new(Duration::ZERO));
        let paused_condvar = Arc::new((Mutex::new(true), Condvar::new()));
        let tick_complete_condvar = Arc::new((Mutex::new(true), Condvar::new()));


        //these are the variables that will be moved to the thread and we lose ownership of them. 
        //they cannot be assigned as the Self{} fields
        // let thread_state = state.clone();
        let thread_elapsed = elapsed.clone();
        let thread_total = total.clone();
        let thread_condvar = paused_condvar.clone();
        let thread_tick_complete_condvar = tick_complete_condvar.clone();

        let handle = thread::spawn(move || {
            let tick = Duration::from_millis(100);

            loop {
                let mut is_paused = thread_condvar.0.lock().unwrap();
                if *is_paused {
                    println!("THread waiting for ispaused condvar");
                    is_paused = thread_condvar.1.wait(is_paused).unwrap();
                }
                drop(is_paused);
                   
                let mut done = thread_tick_complete_condvar.0.lock().unwrap();
                *done = false;
                drop(done);
          
                thread::sleep(tick);

                let is_paused = thread_condvar.0.lock().unwrap();
                if !*is_paused {
                    let mut e = thread_elapsed.lock().unwrap();
                    *e+=tick;

                    let remaining = if *e > *thread_total.lock().unwrap() { Duration::ZERO } else {*thread_total.lock().unwrap() - *e};
                    // println!("{:?}: {}, total: {}",*thread_state.lock().unwrap(), remaining.as_secs(), thread_total.lock().unwrap().as_secs());
                    println!("{}, total: {}", remaining.as_secs(), thread_total.lock().unwrap().as_secs());
                    let _ = app.emit("timer-update",remaining.as_secs());
                    println!("thread: emited event");


                    if *e >= *thread_total.lock().unwrap() {
                        let mut is_paused = thread_condvar.0.lock().unwrap();
                        *is_paused = true;
                        // *thread_state.lock().unwrap() = TimerState::Paused;
                        let _ = app.emit("timer-finished", ());
                        // let _ = app.emit("timer-state-change", TimerState::Paused);
                        let _ = app.emit("timer-state-change", *is_paused);
                    }
                }
                
                drop(is_paused);
                  // 🔁 Re-lock at end to set tick complete
                let mut done = thread_tick_complete_condvar.0.lock().unwrap();
                *done = true;
                thread_tick_complete_condvar.1.notify_all(); // notify setup()
                drop(done); // optional, just to be clear
            }

        });

        Self{
            // state,
            elapsed,
            total,
            paused_condvar,
            tick_complete_condvar,
            handle: Some(handle),
        }
    }       

    pub fn setup(&mut self, total: Duration, app: AppHandle) {
        // *self.state.lock().unwrap() = TimerState::Paused;
        // println!("setup: set state to paused");


        let (is_paused, _) = &*self.paused_condvar;
        *is_paused.lock().unwrap() = true;
        println!("Setup: locked on paused");

        let (lock,cvar) = &*self.tick_complete_condvar;
        let mut done = lock.lock().unwrap();
        println!("setup: locked on tick complete");
        if *done == false{
            println!("setup: waiting for done");
            done = cvar.wait(done).unwrap();
        }
        // *done = false;

        *self.elapsed.lock().unwrap() = Duration::ZERO;
        *self.total.lock().unwrap() = total;
        println!("setup: set new values");
        let _ = app.emit("timer-state-change", *is_paused.lock().unwrap());
        let _ = app.emit("timer-update", total.as_secs());
        println!("setup: emited events");
    }

    pub fn pause(&mut self, app: AppHandle) {
        // *self.state.lock().unwrap() = TimerState::Paused;

        let (is_paused, _) = &*self.paused_condvar;
        *is_paused.lock().unwrap() = true;
        println!("pause: cloekd on paused");

        app.emit("timer-state-change", *is_paused.lock().unwrap());
        // self.state = TimerState::Paused;
    }

    pub fn resume(&mut self, app: AppHandle) {
        // *self.state.lock().unwrap() = TimerState::Running;


        let(lock,cvar) = &*self.paused_condvar;
        let mut paused = lock.lock().unwrap();
        *paused = false;
        cvar.notify_all();

        // app.emit("timer-state-change", TimerState::Running);
        app.emit("timer-state-change", false);
        // self.state = TimerState::Running;
    }

    // pub fn get_elapsed(&self) -> Duration {
    //     *self.elapsed.lock().unwrap()
    // }

    // pub fn get_total(&self) -> Duration {
    //     *self.total.lock().unwrap()
    // }

}

