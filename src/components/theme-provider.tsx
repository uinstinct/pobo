import { ReactNode, useEffect, useState } from "react";

type Theme = "dark" | "light" | "system";

const setDarkTheme = () => {
  document.documentElement.classList.add("dark");
  localStorage.setItem("color-theme", "dark");
};

const setLightTheme = () => {
  document.documentElement.classList.remove("dark");
  localStorage.removeItem("color-theme");
};

export function ThemeProvider({
  defaultTheme = "system",
  children,
}: {
  defaultTheme: Theme;
  children: ReactNode;
}) {
  const [theme, _setTheme] = useState<Theme>(defaultTheme);

  useEffect(() => {
    switch (theme) {
      case "system":
        if (
          localStorage.getItem("color-theme") === "dark" ||
          (!("color-theme" in localStorage) &&
            window.matchMedia("(prefers-color-scheme: dark)").matches)
        ) {
          setDarkTheme();
        }
        break;
      case "dark":
        setDarkTheme();
        break;
      case "light":
        setLightTheme();
        break;
    }
  }, [theme]);

  return children;
}
