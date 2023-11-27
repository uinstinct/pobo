import { UnlistenFn, once } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect } from "react";
import { SessionContextProvider, useSession } from "./session-context";
import SessionStopwatch from "./stopwatch";
import SessionTimer from "./timer";
import TimerInput from "./timer-input";

export default function WrappedSession() {
  return (
    <SessionContextProvider>
      <Session />
    </SessionContextProvider>
  );
}

/**
 * Events and commands from the backend are the actual controllers of state
 * Mostly it will be running in background
 *
 * For example, when the `SessionTimer` finishes, instead of `SessionTimer` switching the `showStopwatch` state, "stopwatch_started" event is listened from the backend
 */
function Session() {
  const { showTimerInput, showTimer, showStopwatch, timerSeconds } =
    useSession();

  useEffect(() => {
    let unlistenStopwatchStart: UnlistenFn = () => {};
    once("stopwatch_started", () => {
      if (showStopwatch.value) return;
      showStopwatch.set(true);
    }).then((_unlisten) => (unlistenStopwatchStart = _unlisten));
    return unlistenStopwatchStart;
  }, []);

  useEffect(() => {
    // TODO: resync_stopwatch
  }, [showStopwatch.value]);

  useEffect(() => {
    invoke<{ elapsed: number | null; timer_seconds: number | null }>(
      "resync_timer"
    ).then((payload) => {
      if (!showTimer.value && payload.elapsed && payload.timer_seconds) {
        timerSeconds.total.set(payload.timer_seconds);
        timerSeconds.current.set(payload.elapsed);
        showTimer.set(true);
      }
    });
  }, [showTimer.value]);

  return (
    <div className="h-screen flex justify-center items-center">
      {showTimerInput.value && <TimerInput />}
      {showTimer.value && <SessionTimer />}
      {showStopwatch.value && <SessionStopwatch />}
    </div>
  );
}
