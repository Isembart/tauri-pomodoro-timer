import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Timer from "./Timer";


type TimerMode = "focus" | "short" | "long" | null;


// let cyclesList = ["focus", "short", "focus", "short", "focus", "long"];

async function setupTimer(totalSecs: number) {
    try {
        await invoke('setup_timer', { totalSecs });
    } catch (error) {
        console.error("Error setting up timer:", error);
    }
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
        <div className={`app-container ${mode}`}>
      <div className="row">
        <button onClick={() => handleSetup(1500, "focus")}>Focus</button>
        <button onClick={() => handleSetup(300, "short")}>Break</button>
        <button onClick={() => handleSetup(900, "long")}>Long Break</button>
      </div>

      <div className="controls">
        {isPaused ? (
          <button
            onClick={() =>
              invoke("resume_timer")
                .then(() => setIsPaused(false))
                .catch((error) =>
                  console.error("Error resuming timer:", error)
                )
            }
          >
            Start
          </button>
        ) : (
          <button
            onClick={() =>
              invoke("pause_timer")
                .then(() => setIsPaused(true))
                .catch((error) => console.error("Error pausing timer:", error))
            }
          >
            Stop
          </button>
        )}

        {/* <button
          onClick={() =>
            invoke("reset_timer")
              .then(() => setIsPaused(true))
              .catch((error) => console.error("Error resetting timer:", error))
          }
        >
          <img
            src="/src/assets/skipIcon.svg"
            alt="Skip Timer"
            style={{ width: "34px", height: "24px" }}
          />
        </button> */}
      </div>

      <Timer timerValue={timerValue} />
    </div>
    );
}