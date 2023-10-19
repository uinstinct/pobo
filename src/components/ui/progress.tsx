import { twMerge } from "tailwind-merge";

export function Progress({
  value,
  className,
}: {
  value: number;
  className?: string;
}) {
  return (
    <div
      className={twMerge(
        "w-full bg-gray-200 rounded-full dark:bg-gray-700",
        className
      )}
    >
      <div
        className="bg-blue-600 text-xs font-medium text-blue-100 text-center p-0.5 leading-none rounded-full transition-all"
        style={{
          width: value + "%",
        }}
      >
        {" "}
        {value}%
      </div>
    </div>
  );
}
