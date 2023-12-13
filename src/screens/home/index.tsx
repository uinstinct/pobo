import { Button } from "@/components/ui/button";
import { useI18nContext } from "@/i18n/i18n-react";
import SettingsIcon from "./settings.svg";

export default function Home() {
  const { LL } = useI18nContext();

  return (
    <>
      <button className="absolute top-2 right-2">
        <img src={SettingsIcon} className="h-6 w-h-6" />
      </button>
      <div className="h-screen flex justify-center items-center flex-col">
        <Button size="large">{LL.START_SESSION()}</Button>
        <Button intent={"secondary"} className="mt-2 rounded-3xl">
          {LL.QUICK_SESSION()}
        </Button>
      </div>
    </>
  );
}
