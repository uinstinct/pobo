import { Dispatch, SetStateAction, useState } from "react";

export type StateValueType<T> = {
  value: T;
  set: Dispatch<SetStateAction<T>>;
};

export function useValueState<T>(initial: T): StateValueType<T> {
  const [value, setValue] = useState(initial);
  return { value, set: setValue };
}
