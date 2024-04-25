import { createFileRoute } from "@tanstack/react-router";

import Header from "@/components/custom/Header";
import SignUpCard from "@/components/custom/SignUpCard";

export const Route = createFileRoute("/signup")({
  component: SignUp,
});

function SignUp() {
  return (
    <div className="mt-8 flex w-full flex-col items-center justify-center gap-y-4">
      <Header />
      <SignUpCard />
    </div>
  );
}
