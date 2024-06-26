import { createFileRoute } from "@tanstack/react-router";

import Header from "@/components/custom/Header";
import LoginCard from "@/components/custom/LoginCard";

export const Route = createFileRoute("/login")({
  component: LogIn,
});

function LogIn() {
  return (
    <div className="flex w-full h-full flex-col items-center justify-center gap-y-4">
      <Header />
      <LoginCard />
    </div>
  );
}
