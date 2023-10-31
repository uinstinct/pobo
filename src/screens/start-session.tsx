import { Button } from "@/components/ui/button";
import { useI18nContext } from "@/i18n/i18n-react";
import { invoke } from "@tauri-apps/api/tauri";

export default function StartSession() {
  const { LL } = useI18nContext();

  const handleStartTimer = () => {
    invoke("start_timer", { timerSeconds: 10 });
  };

  return (
    <div className="h-screen flex justify-center items-center">
      <Button
        intent={"secondary"}
        size="large"
        className="rounded-lg"
        onClick={handleStartTimer}
      >
        {LL.START_SESSION()}
      </Button>
    </div>
  );
}
