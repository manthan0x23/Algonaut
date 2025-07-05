import { themeStore, type ThemeType } from "@/store/theme-store";
import { useStore } from "@tanstack/react-store";
import { useEffect } from "react";
import { Moon, SunDim } from "lucide-react";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";

export const ThemeSwitch = ({ className }: { className?: string }) => {
  const { theme } = useStore(themeStore);

  useEffect(() => {
    const savedTheme =
      document.cookie
        .split("; ")
        .find((row) => row.startsWith("theme="))
        ?.split("=")[1] ?? "light";

    const thusTheme: ThemeType = savedTheme == "dark" ? "dark" : "light";

    themeStore.setState(() => ({ theme: thusTheme }));
    if (savedTheme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }, []);

  const toggleTheme = () => {
    const nextTheme = theme === "dark" ? "light" : "dark";

    document.cookie = `theme=${nextTheme}; path=/; max-age=31536000`;
    themeStore.setState(() => ({
      theme: nextTheme,
    }));

    if (nextTheme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };

  return (
    <div className="flex items-center space-x-2">
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
