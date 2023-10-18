import { Progress } from "@/components/ui/progress";
import TypesafeI18n from "@/i18n/i18n-react";
import { detectLocale } from "@/i18n/i18n-util";
import { loadLocaleAsync } from "@/i18n/i18n-util.async";
import { PropsWithChildren, useEffect, useState } from "react";
import { navigatorDetector } from "typesafe-i18n/detectors";

/**shows loading progress until all requirements for the app are loaded */
export default function AppLoading({ children }: PropsWithChildren) {
  const locale = detectLocale(navigatorDetector);
  const [localesLoaded, setLocalesLoaded] = useState(false);

  useEffect(() => {
    loadLocaleAsync(locale).then(() => setLocalesLoaded(true));
  }, [locale]);

  return localesLoaded ? (
    <div className={`h-screen flex justify-center items-center`}>
      <Progress value={50} className={`w-3/4`} />
    </div>
  ) : (
    <TypesafeI18n locale={locale}>{children}</TypesafeI18n>
  );
}
