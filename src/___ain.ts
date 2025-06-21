import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });

  }
}


// let setup = async () => {await invoke('setup_timer', {totalSecs: 1500});}
// setup();

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

 


  document.querySelector("#start-timer")?.addEventListener("click", async (e) => {
    console.log("Starting timer");
    await invoke('resume_timer');
    
    const startTimerBtn = document.getElementById("start-timer");
    if (startTimerBtn) {
      startTimerBtn.style.display = "none";
    }
    const pauseTimerBtn = document.getElementById("pause-timer");
    if (pauseTimerBtn) {
      pauseTimerBtn.style.display = "block";
    }

  })

  document.querySelector("#pause-timer")?.addEventListener("click", async (e) => {
    console.log("Starting timer");
    await invoke('pause_timer');

      const startTimerBtn = document.getElementById("start-timer");
    if (startTimerBtn) {
      startTimerBtn.style.display = "block";
    }
    const pauseTimerBtn = document.getElementById("pause-timer");
    if (pauseTimerBtn) {
      pauseTimerBtn.style.display = "none";
    }
  })


  document.getElementById("focus-btn")?.addEventListener("click", async () => {
    await invoke("setup_timer", {totalSecs: 1500});
  })
  document.getElementById("break-btn")?.addEventListener("click", async () => {
    await invoke("setup_timer", {totalSecs: 5});
  })
  document.getElementById("lbreak-btn")?.addEventListener("click", async () => {
    await invoke("setup_timer", {totalSecs: 900});
  })
  
});

listen<number>('timer-update', event => {
  const remaining_seconds = event.payload;
  const timerEl = document.querySelector("#timer");
  if (timerEl) {
    timerEl.textContent = String(remaining_seconds);
  }
})
