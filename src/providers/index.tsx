import { ThemeProvider } from "./Theme";
import RouterProvider from "./Router";
import { Toaster } from "@/components/ui/toaster.tsx";

export default function Providers() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <RouterProvider />
      <Toaster />
    </ThemeProvider>
  );
}
