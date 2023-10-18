import { ThemeProvider } from "@/components/theme-provider";
import AppLoading from "./app-loading";
import StartSession from "./start-session";

export default function App() {
  return (
    <ThemeProvider defaultTheme="dark">
      <AppLoading>
        <StartSession />
      </AppLoading>
    </ThemeProvider>
  );
}
