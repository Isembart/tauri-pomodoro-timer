use std::{
    process::exit,
    sync::{Arc, Condvar, Mutex},
    thread::{self, sleep, JoinHandle},
    time::{Duration, Instant},
};

use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;

pub struct Timer {
    elapsed: Arc<Mutex<Duration>>,
    total: Arc<Mutex<Duration>>,
    paused_condvar: Arc<(Mutex<bool>, Condvar)>,
    tick_complete_condvar: Arc<(Mutex<bool>, Condvar)>,
    handle: Option<JoinHandle<()>>,
}

//for some reason rust-analyzer won't count usage in lib.rs' tauri commands so without it we get shit ton of warning
#[allow(dead_code)]
impl Timer {
    pub fn new(app: AppHandle) -> Self {
        let elapsed = Arc::new(Mutex::new(Duration::ZERO));
        let total = Arc::new(Mutex::new(Duration::ZERO));
        let paused_condvar = Arc::new((Mutex::new(true), Condvar::new()));
        let tick_complete_condvar = Arc::new((Mutex::new(true), Condvar::new()));

        //these are the variables that will be moved to the thread and we lose ownership of them.
        //they cannot be assigned as the Self{} fields
        let thread_elapsed = elapsed.clone();
        let thread_total = total.clone();
        let thread_condvar = paused_condvar.clone();
        let thread_tick_complete_condvar = tick_complete_condvar.clone();

        let handle = thread::spawn(move || {
            let tick = Duration::from_millis(100);

            loop {
                let mut done = thread_tick_complete_condvar.0.lock().unwrap();
                *done = false;

                thread::sleep(tick);

                let mut is_paused = thread_condvar.0.lock().unwrap();
                if !*is_paused {
                    let mut e = thread_elapsed.lock().unwrap();
                    *e += tick;

                    let remaining = if *e > *thread_total.lock().unwrap() {
                        Duration::ZERO
                    } else {
                        *thread_total.lock().unwrap() - *e
                    };
                    println!(
                        "{}, total: {}",
                        remaining.as_secs(),
                        thread_total.lock().unwrap().as_secs()
                    );
                    let _ = app.emit("timer-update", remaining.as_secs());

                    if *e >= *thread_total.lock().unwrap() {
                        // let mut is_paused = thread_condvar.0.lock().unwrap();
                        *is_paused = true;
                        let _ = app.emit("timer-finished", ());
                        let _ = app.emit("timer-state-change", *is_paused);
                        println!("Timer finished!");


                        app.notification().builder().title("Session finished!").body("You finished your next session, keep it up!").show();
                    }
                    *done = true;
                    thread_tick_complete_condvar.1.notify_all(); // notify setup() that if it changes something it wont be overwritten by running thread
                } else {
                    *done = true;
                    thread_tick_complete_condvar.1.notify_all(); // notify setup() that if it changes something it wont be overwritten by running thread

                    //we need to drop done before waiting because if we don't do so the setup() function won't be able to check if done is true or false
                    //because the thread will still have the lock, because the thread will be waiting for setup and setup will be waiting for locking done we'll have a starvation of two threads
                    drop(done);

                    is_paused = thread_condvar.1.wait(is_paused).unwrap();
                }
                drop(is_paused);
            }
        });

        Self {
            elapsed,
            total,
            paused_condvar,
            tick_complete_condvar,
            handle: Some(handle),
        }
    }

    pub fn setup(&mut self, total: Duration, app: AppHandle) {
        let (is_paused, _) = &*self.paused_condvar;
        *is_paused.lock().unwrap() = true;

        let (lock, cvar) = &*self.tick_complete_condvar;
        let mut done = lock.lock().unwrap();
        if *done == false {
            done = cvar.wait(done).unwrap();
            drop(done); //just so i dont get the warning that i dont use done
        }
        // *done = false;

        *self.elapsed.lock().unwrap() = Duration::ZERO;
        *self.total.lock().unwrap() = total;
        let _ = app.emit("timer-state-change", *is_paused.lock().unwrap());
        let _ = app.emit("timer-update", total.as_secs());
    }

    pub fn pause(&mut self, app: AppHandle) {
        let (is_paused, _) = &*self.paused_condvar;
        *is_paused.lock().unwrap() = true;

        let _ = app.emit("timer-state-change", *is_paused.lock().unwrap());
    }

    pub fn resume(&mut self, app: AppHandle) {
        let (lock, cvar) = &*self.paused_condvar;
        let mut paused = lock.lock().unwrap();
        *paused = false;
        cvar.notify_all();

        let _ = app.emit("timer-state-change", *paused);
    }

    pub fn get_remaining(&self) -> Duration{
        let elapsed = self.elapsed.lock().unwrap();
        let total = self.total.lock().unwrap();
        *total - *elapsed
    }
}
