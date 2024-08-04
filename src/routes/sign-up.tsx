import { createFileRoute } from "@tanstack/react-router";

import Header from "@/components/custom/Header";
import SignUpCard from "@/components/custom/SignUpCard";

export const Route = createFileRoute("/sign-up")({
  component: SignUp,
});

function SignUp() {
  return (
    <div className="flex h-full w-full flex-col items-center justify-center gap-y-4 bg-neutral-100 dark:bg-neutral-800">
      <Header />
      <SignUpCard />
    </div>
  );
}
