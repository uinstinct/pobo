import { Button } from "@/components/ui/button";
import { useI18nContext } from "@/i18n/i18n-react";

export default function StartSession() {
  const { LL } = useI18nContext();
  return <Button>{LL.START_SESSION()}</Button>;
}
