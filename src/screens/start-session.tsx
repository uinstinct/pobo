import { Button } from "@/components/ui/button";
import { useI18nContext } from "@/i18n/i18n-react";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect } from "react";

export default function StartSession() {
  const { LL } = useI18nContext();

  const handleStartTimer = () => {
    invoke("start_timer");
  };

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    (async () => {
      unlisten = await listen("time_left", (event) => {
        console.log("the event details->", event);
      });
    })();

    return () => unlisten?.();
  }, []);

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
