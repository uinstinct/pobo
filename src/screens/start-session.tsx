import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { useI18nContext } from "@/i18n/i18n-react";

export default function StartSession() {
  const { LL } = useI18nContext();
  return (
    <div>
      <Button>{LL.START_SESSION()}</Button>
      <Progress value={20} />
    </div>
  );
}
