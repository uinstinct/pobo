import { ThemeProvider } from "@/components/theme-provider";
import AppLoading from "./app-loading";
import Home from "./home";

export default function App() {
  return (
    <ThemeProvider defaultTheme="dark">
      <AppLoading>
        <Home />
      </AppLoading>
    </ThemeProvider>
  );
}
