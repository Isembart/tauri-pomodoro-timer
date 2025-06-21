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

    const [timerRunning, setTimeRunning] = useState(false);
    const [timerValue, setTimerValue] = useState(0);

    useEffect(() => {
        const setup = async () => {
            listen<number>("timer-update", (event) => {
                console.log("Timer updated:", event.payload);
                setTimerValue(event.payload);
            })

            listen("timer-state-change", (event) => {
                console.log("Time state changed:", event.payload);
                setTimeRunning(event.payload === "Running");
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

            {timerRunning ? (
                <button onClick={() => {
                    invoke('pause_timer').then(() => setTimeRunning(false)).catch(error => {
                        console.error("Error pausing timer:", error);
                    });
                }}>Stop</button>
            ) : (
                <button onClick={() => {
                    invoke('resume_timer').then(() => setTimeRunning(true)).catch(error => {
                        console.error("Error resuming timer:", error);
                    });
                }}>Start</button>
            )}
            
            {/* <h1></h1> */}
            <Timer timerValue={timerValue}/>
        </div>
    );
}