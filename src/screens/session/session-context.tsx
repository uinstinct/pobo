import { StateValueType, useValueState } from "@/utils/state";
import { ReactNode, createContext, useContext } from "react";

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
        showTimerInput,

        showTimer,
        timerSeconds: {
          current: timerSecondsCurrent,
          total: timerSecondsTotal,
        },

        showStopwatch,
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
