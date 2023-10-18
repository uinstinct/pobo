import { ReactNode, useEffect, useState } from "react";

type Theme = "dark" | "light" | "system";
type ThemeState = "dark" | "light";

export function ThemeProvider({
  defaultTheme,
  children,
}: {
  defaultTheme?: Theme;
  children: ReactNode;
}) {
  const [theme, setTheme] = useState<ThemeState>("light");

  useEffect(() => {
    if (defaultTheme !== "system") return;

    if (
      localStorage.getItem("color-theme") === "dark" ||
      (!("color-theme" in localStorage) &&
        window.matchMedia("(prefers-color-scheme: dark)").matches)
    ) {
      setTheme("dark");
    } else {
      setTheme("light");
    }
  }, []);

  useEffect(() => {
    if (theme === "dark") {
      document.documentElement.classList.add("dark");
      localStorage.setItem("color-theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.removeItem("color-theme");
    }
  }, [theme]);

  return children;
}
