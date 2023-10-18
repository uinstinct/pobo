import { PropsWithChildren } from "react";

export function Progress({
  children,
}: PropsWithChildren<{ value: number; className: string }>) {
  return (
    <div className="w-full bg-gray-200 rounded-full dark:bg-gray-700">
      <div
        className="bg-blue-600 text-xs font-medium text-blue-100 text-center p-0.5 leading-none rounded-full"
        style={{ width: "45%" }}
      >
        {" "}
        45%
      </div>
    </div>
  );
}
