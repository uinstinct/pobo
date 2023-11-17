import { Stopwatch } from "@/components/timer";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { useEffect, useRef } from "react";
import { useSession } from "./session-context";

/**
 * It is automatically invoked when the stopwatch finishes.
 *
 * _Cooldown_ is compulsory after a `SessionTimer`.
 */
export default function SessionStopwatch() {
  const { stopwatchSeconds } = useSession();

  const intervalRef = useRef<ReturnType<typeof setInterval> | undefined>(
    undefined
  );

  const startStopwatchInterval = () => {
    if (intervalRef.current) clearInterval(intervalRef.current);

    intervalRef.current = setInterval(() => {
      stopwatchSeconds.current.set((prevCurrentSecs) => prevCurrentSecs + 1);
    }, 1_000);
  };

  useEffect(() => {
    startStopwatchInterval();
    return () => {
      clearInterval(intervalRef.current);
    };
  }, []);

  useEffect(() => {
    let unlistenTimerFinish: UnlistenFn = () => {};
    listen("stopwatch_finished", () =>
      console.log("stopwatch was finished")
    ).then((_unlistenFn) => (unlistenTimerFinish = _unlistenFn));
    return unlistenTimerFinish;
  }, []);

  return (
    <Stopwatch currentSecs={stopwatchSeconds.current.value} totalSecs={0} />
  );
}
