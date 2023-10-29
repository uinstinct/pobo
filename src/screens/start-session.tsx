import { Button } from "@/components/ui/button";
import { useI18nContext } from "@/i18n/i18n-react";

export default function StartSession() {
  const { LL } = useI18nContext();
  return (
    <div className="h-screen flex justify-center items-center">
      <Button intent={"secondary"} size="large" className="rounded-lg">
        {LL.START_SESSION()}
      </Button>
    </div>
  );
}
