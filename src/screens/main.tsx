import { ThemeProvider } from "@/components/theme-provider";
import { Button } from "@/components/ui/button";

export default function App() {
  return (
    <ThemeProvider defaultTheme="dark">
      <Button size={"lg"} variant={"destructive"}>
        Start Session
      </Button>
    </ThemeProvider>
  );
}
