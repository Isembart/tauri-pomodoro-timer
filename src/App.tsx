import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Timer from "./Timer";


async function setupTimer(totalSecs: number) {
    try {
        await invoke('setup_timer', { totalSecs });
    } catch (error) {
        console.error("Error setting up timer:", error);
    }
}

export default function App() {

    const [isPaused, setIsPaused] = useState(true);
    const [timerValue, setTimerValue] = useState(0);

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
        <div>
            <div className="row">
                <button onClick={()=>setupTimer(1500)}>Focus</button>
                <button onClick={()=>setupTimer(5)}>Break</button>
                <button onClick={()=>setupTimer(900)}>Long Break</button>
            </div>

            {isPaused ? (
                <button onClick={() => {
                    invoke('resume_timer').then(() => setIsPaused(false)).catch(error => {
                        console.error("Error resuming timer:", error);
                    });
                }}>Start</button>
            ) : (
                <button onClick={() => {
                    invoke('pause_timer').then(() => setIsPaused(true)).catch(error => {
                        console.error("Error pausing timer:", error);
                    });
                }}>Stop</button>
            )}
            
            {/* <h1></h1> */}
            <Timer timerValue={timerValue}/>
        </div>
    );
}