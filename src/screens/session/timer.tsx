import { Timer } from "@/components/timer";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { useEffect, useRef } from "react";
import { useSession } from "./session-context";

export default function SessionTimer() {
  const { timerSeconds } = useSession();

  const intervalRef = useRef<ReturnType<typeof setInterval> | undefined>(
    undefined
  );
  const stopTimer = () => {
    clearInterval(intervalRef.current);
  };

  const startTimerInterval = () => {
    if (intervalRef.current) stopTimer();

    intervalRef.current = setInterval(() => {
      timerSeconds.current.set((prevCurrentSecs) => prevCurrentSecs + 1);
    }, 1_000);
  };

  useEffect(() => {
    let unlistenTimerFinish: UnlistenFn = () => {};
    listen("timer_finished", stopTimer).then(
      (_unlistenFn) => (unlistenTimerFinish = _unlistenFn)
    );

    return unlistenTimerFinish;
  }, []);

  useEffect(() => {
    if (!timerSeconds.total.value) return;
    startTimerInterval();
  }, [timerSeconds.total.value]);

  return (
    <Timer
      currentSecs={timerSeconds.current.value}
      totalSecs={timerSeconds.total.value!}
    />
  );
}
