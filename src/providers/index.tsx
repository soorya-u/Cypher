import { ThemeProvider } from "./Theme";
import RouterProvider from "./Router";

export default function Providers() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <RouterProvider />
    </ThemeProvider>
  );
}
