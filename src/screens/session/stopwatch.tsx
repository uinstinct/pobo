import { Stopwatch } from "@/components/timer";
import { Button } from "@/components/ui/button";
import { useI18nContext } from "@/i18n/i18n-react";
import { useValueState } from "@/utils/state";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef } from "react";
import { useSession } from "./session-context";

/**
 * It is automatically invoked when the stopwatch finishes.
 *
 * _Cooldown_ is compulsory after a `SessionTimer`.
 */
export default function SessionStopwatch() {
  const { LL } = useI18nContext();
  const { stopwatchSeconds, showTimerInput } = useSession();
  const isStopwatchCompleted = useValueState(false);

  const intervalRef = useRef<ReturnType<typeof setInterval> | undefined>(
    undefined
  );

  const stopStopwatch = () => {
    if (intervalRef.current) clearInterval(intervalRef.current);
  };

  const startStopwatchInterval = () => {
    stopStopwatch();

    intervalRef.current = setInterval(() => {
      stopwatchSeconds.current.set((prevCurrentSecs) => prevCurrentSecs + 1);
    }, 1_000);
  };

  const manuallyStopStopwatch = () => {
    invoke<null>("stop_stopwatch").then(() => {
      stopStopwatch();
      showTimerInput.set(true);
    });
  };

  useEffect(() => {
    startStopwatchInterval();
    return () => {
      clearInterval(intervalRef.current);
    };
  }, []);

  useEffect(() => {
    let unlistenTimerFinish: UnlistenFn = () => {};
    listen("stopwatch_finished", () => isStopwatchCompleted.set(true)).then(
      (_unlistenFn) => (unlistenTimerFinish = _unlistenFn)
    );
    return unlistenTimerFinish;
  }, []);

  return (
    <div>
      <Stopwatch currentSecs={stopwatchSeconds.current.value} totalSecs={0} />
      <div className="flex justify-center m-5">
        <Button
          intent={isStopwatchCompleted.value ? "success" : "danger"}
          size="large"
          className="rounded-lg"
          onClick={() =>
            isStopwatchCompleted.value
              ? showTimerInput.set(true)
              : manuallyStopStopwatch()
          }
        >
          {isStopwatchCompleted.value ? LL.NEXT_SESSION() : LL.STOP_COOLDOWN()}
        </Button>
      </div>
    </div>
  );
}
