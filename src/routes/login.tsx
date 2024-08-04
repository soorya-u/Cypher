import { createFileRoute } from "@tanstack/react-router";

import Header from "@/components/custom/Header";
import LoginCard from "@/components/custom/LoginCard";

export const Route = createFileRoute("/login")({
  component: LogIn,
});

function LogIn() {
  return (
    <div className="flex h-full w-full flex-col items-center justify-center gap-y-4 bg-neutral-100 dark:bg-neutral-800">
      <Header />
      <LoginCard />
    </div>
  );
}
