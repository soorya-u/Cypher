import { Moon, Sun, Monitor } from "lucide-react";

import { useTheme } from "@/hooks/useTheme";

export default function ThemeToggler() {
  const { theme, setTheme } = useTheme();

  return (
    <button onClick={() => setTheme("system")} className="bg-primary text-blue-500">
      Set to {theme}
    </button>
  );
}
