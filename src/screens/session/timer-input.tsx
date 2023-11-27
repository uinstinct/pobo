import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useI18nContext } from "@/i18n/i18n-react";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { useSession } from "./session-context";

function HoursMinsSecsInput({
  onTimeChange,
}: {
  onTimeChange: (seconds: number) => void;
}) {
  const [hours, setHours] = useState("");
  const [minutes, setMinutes] = useState("");
  const [seconds, setSeconds] = useState("");

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

export default function TimerInput() {
  const { LL } = useI18nContext();
  const { timerSeconds, showTimer } = useSession();
  return (
    <div className="flex flex-col justify-center">
      <Button
        intent={"secondary"}
        size="large"
        className="rounded-lg"
        onClick={() => {
          invoke("start_timer", { timerSeconds: timerSeconds.total.value });
          showTimer.set(true);
        }}
      >
        {LL.START_SESSION()}
      </Button>
      <HoursMinsSecsInput
        onTimeChange={(secs) => timerSeconds.total.set(secs)}
      />
    </div>
  );
}
