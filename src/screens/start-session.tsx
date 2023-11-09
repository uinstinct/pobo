import Timer from "@/components/timer";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useI18nContext } from "@/i18n/i18n-react";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef, useState } from "react";

function TimerInput({
  onTimeChange,
}: {
  onTimeChange: (seconds: number) => void;
}) {
  const [hours, setHours] = useState<number | undefined>(undefined);
  const [minutes, setMinutes] = useState<number | undefined>(undefined);
  const [seconds, setSeconds] = useState<number | undefined>(undefined);

  const handleTimeChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value.trim();
    if (isNaN(value as any)) return 0;
    return +value;
  };

  useEffect(() => {
    if (hours || minutes || seconds) {
      onTimeChange(
        (hours || 0) * 60 * 60 + (minutes || 0) * 60 + (seconds || 0)
      );
    }
  }, [hours, minutes, seconds]);

  return (
    <div className="mt-6 grid grid-cols-3 gap-2">
      <Input
        type="number"
        placeholder="Hours"
        value={hours}
        onChange={(event) => setHours(handleTimeChange(event))}
      />
      <Input
        type="number"
        placeholder="Minutes"
        value={minutes}
        onChange={(event) => setMinutes(handleTimeChange(event))}
      />
      <Input
        type="number"
        placeholder="Seconds"
        value={seconds}
        onChange={(event) => setSeconds(handleTimeChange(event))}
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
  const clearLocalInterval = () => clearInterval(intervalRef.current);

  const handleStartTimer = () => {
    if (!totalSecs) return;
    invoke("start_timer", { timerSeconds: totalSecs }).then(clearLocalInterval);

    setShowTimer(true);

    intervalRef.current = setInterval(() => {
      setCurrentSecs(
        /**
         * clear the interval if the seconds have elapsed
         * although this should not happen, it will act as a fallback
         */
        (prevCurrentSecs) => {
          console.log(
            "the mode was",
            import.meta.env.MODE,
            "and prod was",
            import.meta.env.PROD,
            "and dev was",
            import.meta.env.DEV
          );
          if (import.meta.env.PROD && prevCurrentSecs + 1 > totalSecs) {
            clearLocalInterval();
            return prevCurrentSecs;
          }
          return prevCurrentSecs + 1;
        }
      );
    }, 1_000);
  };

  useEffect(() => {
    return clearLocalInterval;
  }, [intervalRef]);

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
