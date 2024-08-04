export type Theme = "dark" | "light";

export type ThemeProviderState = {
  theme: Theme;
  setTheme: (theme: Theme) => void;
};
