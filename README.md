# tauri-pomodoro-timer
![image](showcase.png)

Basic pomodoro timer written using Rust & Tauri just because there's not much free alternatives in windows store and I don't want to have my browser open all the time.

Supports notifications
# Start dev

```
pnpm tauri dev
```
Be aware that because of tauri single_instance plugin you're not able to use dev version if you have a built version running already.

# Build 
```
pnpm tauri build
```
