import { SidebarClickable } from "@/components/ui/sidebar";
import { useTheme } from "@/hooks/use-theme";
import { cn } from "@/utils/cn";
import { IconMoon, IconSun } from "@tabler/icons-react";
import { Link } from "@tanstack/react-router";

export function ThemeToggler() {
  const { theme, setTheme } = useTheme();
  const handleClick = () => setTheme(theme === "dark" ? "light" : "dark");
  return (
    <SidebarClickable
      link={{
        handleClick,
        icon:
          theme === "dark" ? (
            <IconMoon className="h-5 w-5 flex-shrink-0 [&_path]:!text-neutral-700 dark:[&_path]:!text-neutral-200" />
          ) : (
            <IconSun className="h-5 w-5 flex-shrink-0 [&_path]:!text-neutral-700 dark:[&_path]:!text-neutral-200" />
          ),
        label: `${theme} mode`,
      }}
    />
  );
}

export function IndependentThemeToggler() {
  const { theme, setTheme } = useTheme();
  const handleClick = () => setTheme(theme === "dark" ? "light" : "dark");
  return (
    <Link
      onClick={handleClick}
      className={cn(
        "absolute -right-7 -top-2 flex items-center justify-start gap-2 py-2",
      )}
    >
      {theme === "dark" ? (
        <IconMoon className="h-5 w-5 flex-shrink-0 [&_path]:!text-neutral-700 dark:[&_path]:!text-neutral-200" />
      ) : (
        <IconSun className="h-5 w-5 flex-shrink-0 [&_path]:!text-neutral-700 dark:[&_path]:!text-neutral-200" />
      )}
    </Link>
  );
}
