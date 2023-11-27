import { StateValueType, useValueState } from "@/utils/state";
import { ReactNode, SetStateAction, createContext, useContext } from "react";

interface ISessionContext {
  showTimerInput: StateValueType<boolean>;

  showTimer: StateValueType<boolean>;
  timerSeconds: {
    current: StateValueType<number>;
    total: StateValueType<number | null>;
  };

  showStopwatch: StateValueType<boolean>;
  stopwatchSeconds: {
    current: StateValueType<number>;
  };
}

const SessionContext = createContext({} as ISessionContext);

export function SessionContextProvider({ children }: { children: ReactNode }) {
  const showTimerInput = useValueState(true);

  const showTimer = useValueState(false);

  const timerSecondsCurrent = useValueState(0);
  const timerSecondsTotal = useValueState<number | null>(null);

  const showStopwatch = useValueState(false);
  const stopwatchSecondsCurrent = useValueState(0);

  return (
    <SessionContext.Provider
      value={{
        showTimerInput: {
          value: showTimerInput.value,
          set: (value: SetStateAction<boolean>) => {
            showTimerInput.set(value);
            showTimer.set(!value);
            showStopwatch.set(!value);
          },
        },

        showTimer: {
          value: showTimer.value,
          set: (value: SetStateAction<boolean>) => {
            showTimer.set(value);
            showTimerInput.set(!value);
            showStopwatch.set(!value);
          },
        },
        timerSeconds: {
          current: timerSecondsCurrent,
          total: timerSecondsTotal,
        },

        showStopwatch: {
          value: showStopwatch.value,
          set: (value: SetStateAction<boolean>) => {
            showStopwatch.set(value);
            showTimer.set(!value);
            showTimerInput.set(!value);
          },
        },
        stopwatchSeconds: {
          current: stopwatchSecondsCurrent,
        },
      }}
    >
      {children}
    </SessionContext.Provider>
  );
}

export function useSession() {
  return useContext(SessionContext);
}
