import { createFileRoute } from "@tanstack/react-router";

import Header from "@/components/custom/Header";
import LoginCard from "@/components/custom/LoginCard";

export const Route = createFileRoute("/login")({
  component: LogIn,
});

function LogIn() {
  return (
    <div className="w-full flex flex-col justify-center items-center mt-8 gap-y-4">
      <Header />
      <LoginCard />
    </div>
  );
}
