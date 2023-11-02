interface ITimerProps {
  currentSecs: number;
  totalSecs: number;
}

/*
export default function Timer() {
  // const step =

  return (
    <div>
      <svg height={"100%"} width={"100%"} xmlns="http://www.w3.org/2000/svg">
        <circle
          id="circle"
          r="40"
          cx="50"
          cy="50"
          strokeLinecap="round"
          strokeWidth="10"
          stroke="#6fdb6f"
          fill="none"
          strokeDashoffset={0}
          className="transition duration-1000 ease-linear"
        />
      </svg>
    </div>
  );
}
*/

function formattedTime(duration: number) {
  const integerDuration = Math.floor(duration);
  return integerDuration < 10 ? `0${integerDuration}` : `${integerDuration}`;
}

export default function Timer({ currentSecs, totalSecs }: ITimerProps) {
  const remainingSecs = totalSecs - currentSecs;
  const hours = remainingSecs / (60 * 60);
  const mins = remainingSecs / 60;
  const secs = remainingSecs % 60;
  return (
    <div>
      <span className="text-6xl text-gray-900 dark:text-white">
        {formattedTime(hours)} : {formattedTime(mins)} : {formattedTime(secs)}
      </span>
    </div>
  );
}
