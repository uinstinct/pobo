import { Timer } from "@/components/timer";
import { Button } from "@/components/ui/button";
import { useI18nContext } from "@/i18n/i18n-react";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef } from "react";
import { useSession } from "./session-context";

export default function SessionTimer() {
  const { LL } = useI18nContext();
  const { timerSeconds } = useSession();

  const intervalRef = useRef<ReturnType<typeof setInterval> | undefined>(
    undefined
  );
  const stopTimer = () => {
    clearInterval(intervalRef.current);
  };
  const manuallyStopTimer = () => {
    invoke<null>("stop_timer").then(stopTimer);
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
    <div>
      <Timer
        currentSecs={timerSeconds.current.value}
        totalSecs={timerSeconds.total.value!}
      />
      <div className="flex justify-center m-5">
        <Button
          intent={"danger"}
          size="large"
          className="rounded-lg"
          onClick={manuallyStopTimer}
        >
          {LL.STOP_SESSION()}
        </Button>
      </div>
    </div>
  );
}
