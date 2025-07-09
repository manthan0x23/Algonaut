import { useEffect } from "react";
import { Moon, SunDim } from "lucide-react";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { useThemeStore, type ThemeType } from "@/store/theme-store";
import { cn } from "@/lib/utils";

export const ThemeSwitch = ({ className }: { className?: string }) => {
  const theme = useThemeStore((state) => state.theme);
  const setTheme = useThemeStore((state) => state.setTheme);

  useEffect(() => {
    const savedTheme =
      document.cookie
        .split("; ")
        .find((row) => row.startsWith("theme="))
        ?.split("=")[1] ?? "dark";

    const thusTheme: ThemeType = savedTheme == "dark" ? "dark" : "light";

    setTheme(thusTheme);
    if (savedTheme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }, []);

  const toggleTheme = () => {
    const nextTheme = theme === "dark" ? "light" : "dark";

    document.cookie = `theme=${nextTheme}; path=/; max-age=31536000`;
    setTheme(nextTheme);

    if (nextTheme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };

  return (
    <div className={cn("flex items-center space-x-2", className)}>
      <Label htmlFor="airplane-mode" className="capitalize">
        {theme === "dark" ? <Moon size={17} /> : <SunDim size={17} />}
      </Label>
      <Switch
        checked={theme == "light"}
        onClick={toggleTheme}
        title="Toggle theme"
        className="cursor-pointer"
      />
    </div>
  );
};
