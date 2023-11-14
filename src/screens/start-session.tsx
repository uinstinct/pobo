import Timer from "@/components/timer";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useI18nContext } from "@/i18n/i18n-react";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef, useState } from "react";

function TimerInput({
  onTimeChange,
}: {
  onTimeChange: (seconds: number) => void;
}) {
  const [hours, setHours] = useState<string>("");
  const [minutes, setMinutes] = useState<string>("");
  const [seconds, setSeconds] = useState<string>("");

  const handleTimeChange = (
    event: React.ChangeEvent<HTMLInputElement>,
    prevTimeUnit: string
  ) => {
    const value = event.target.value.trim();
    if (isNaN(value as any)) return prevTimeUnit;
    else if (+value >= 60) return prevTimeUnit;
    return value;
  };

  useEffect(() => {
    if (hours || minutes || seconds) {
      onTimeChange(
        (+hours || 0) * 60 * 60 + (+minutes || 0) * 60 + (+seconds || 0)
      );
    }
  }, [hours, minutes, seconds]);

  return (
    <div className="mt-6 grid grid-cols-3 gap-2">
      <Input
        placeholder="Hours"
        value={hours}
        onChange={(event) =>
          setHours((prevHours) => handleTimeChange(event, prevHours))
        }
      />
      <Input
        placeholder="Minutes"
        value={minutes}
        onChange={(event) =>
          setMinutes((prevMinutes) => handleTimeChange(event, prevMinutes))
        }
      />
      <Input
        placeholder="Seconds"
        value={seconds}
        onChange={(event) =>
          setSeconds((prevSeconds) => handleTimeChange(event, prevSeconds))
        }
      />
    </div>
  );
}

export default function StartSession() {
  const [currentSecs, setCurrentSecs] = useState(0);
  const [totalSecs, setTotalSecs] = useState<number | null>(null);
  const [showTimer, setShowTimer] = useState(false);
  const { LL } = useI18nContext();

  const intervalRef = useRef<ReturnType<typeof setInterval> | undefined>(
    undefined
  );
  const stopTimer = () => {
    setShowTimer(false);
    clearInterval(intervalRef.current);
  };

  const startTimerInterval = () => {
    setShowTimer(true);

    if (intervalRef.current) stopTimer();

    intervalRef.current = setInterval(() => {
      setCurrentSecs(
        /**
         * clear the interval if the seconds have elapsed
         * although this should not happen, it will act as a fallback
         */
        (prevCurrentSecs) => {
          if (import.meta.env.PROD && prevCurrentSecs + 1 > totalSecs!) {
            stopTimer();
            return prevCurrentSecs;
          }
          return prevCurrentSecs + 1;
        }
      );
    }, 1_000);
  };

  const handleStartTimer = () => {
    if (!totalSecs) return;
    invoke("start_timer", { timerSeconds: totalSecs }).then(() =>
      console.log("starting the timer finished")
    );
    startTimerInterval();
  };

  useEffect(() => {
    let unlisten: UnlistenFn = () => {};
    listen("timer_finished", stopTimer).then(
      (_unlistenFn) => (unlisten = _unlistenFn)
    );
    return unlisten;
  }, []);

  useEffect(() => {
    if (showTimer) return;

    invoke<{ elapsed: number | null; timer_seconds: number | null }>(
      "resync_timer"
    ).then((payload) => {
      console.log("the payload was", payload);
      if (!showTimer && payload.elapsed && payload.timer_seconds) {
        setTotalSecs(payload.timer_seconds);
        setCurrentSecs(payload.elapsed);
        startTimerInterval();
      }
    });
  }, [showTimer]);

  return (
    <div className="h-screen flex justify-center items-center">
      {!showTimer ? (
        <div className="flex flex-col justify-center">
          <Button
            intent={"secondary"}
            size="large"
            className="rounded-lg"
            onClick={handleStartTimer}
          >
            {LL.START_SESSION()}
          </Button>
          <TimerInput onTimeChange={(secs) => setTotalSecs(secs)} />
        </div>
      ) : (
        <Timer currentSecs={currentSecs} totalSecs={totalSecs!} />
      )}
    </div>
  );
}
