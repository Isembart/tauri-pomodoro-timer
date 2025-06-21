
export default function Timer({ timerValue }: { timerValue: number }) {
    //convert seoncd to mm:ss format
    // console.log("Timer value:", timerValue);
    const formatTime = (seconds: number): string => {
        const minutes = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
    };

    return (
        <div>
            <h1>{formatTime(timerValue)}</h1>
        </div>
    )
}