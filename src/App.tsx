import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Timer from "./components/Timer";


type TimerMode = "focus" | "short" | "long" | null;


// let cyclesList = ["focus", "short", "focus", "short", "focus", "long"];

async function setupTimer(totalSecs: number) {
    try {
        await invoke('setup_timer', { totalSecs });
    } catch (error) {
        console.error("Error setting up timer:", error);
    }
}


function playTimerSound(number: number = 1) {
    const audio = new Audio(`/sounds/ClockTick${number}.mp3`);
    audio.play().catch((error) => {
        console.error("Error playing timer sound:", error);
    });
}


export default function App() {

    const hasInitialized = useRef(false);
    const [isPaused, setIsPaused] = useState(true);
    const [timerValue, setTimerValue] = useState(0);
    const [mode, setMode] = useState<TimerMode>(null);

      const handleSetup = (seconds: number, selectedMode: TimerMode) => {
        setupTimer(seconds);
        setMode(selectedMode);
    };

    useEffect(() => {

      const setup2 = async () => {
        if (!hasInitialized.current) {
            handleSetup(1500, "focus");
            hasInitialized.current = true;
            let remaining_time = await invoke<number>("get_remaining");
            setTimerValue(remaining_time);
        }
      }

      setup2().catch(error => {
          console.error("Error during initial setup:", error);  
      });
        

        return () => {
        };
    }, []);

    useEffect(() => {
        const setup = async () => {
            listen<number>("timer-update", (event) => {
                setTimerValue(event.payload);
            })

            listen<boolean>("timer-state-change", (event) => {
                console.log("Timer state changed:", event.payload);
                setIsPaused(event.payload);
            })
            
            
        };
        setup().catch(error => {
            console.error("Error setting up event listener:", error);
        });

        return () => {
            // Cleanup if necessary
        };
    },[]);

    return (
      <div data-tauri-drag-region className={`app-container ${mode}`}>
        <Timer timerValue={timerValue} />
        <div className="row">
          <button onClick={() => handleSetup(1500, "focus")}>Focus</button>
          <button onClick={() => handleSetup(5, "short")}>Break</button>
          <button onClick={() => handleSetup(900, "long")}>Long Break</button>
        </div>


        <button className={`MainButton isPaused-${isPaused}`} onClick={() => {
          switch(isPaused) {
            case true:
              invoke("resume_timer")
                .then(() => {
                  playTimerSound(1);
                })
                break;
            case false:
              invoke("pause_timer")
                .then(() => {
                  playTimerSound(1);
                })
                break;
          }
          setIsPaused(!isPaused);
        }}>
        {isPaused ? "Start" : "Pause"}
        </button>

      </div>
    );
}