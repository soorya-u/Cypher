import { createFileRoute } from "@tanstack/react-router";

import Header from "@/components/custom/Header";
import SignUpCard from "@/components/custom/SignUpCard";

export const Route = createFileRoute("/signup")({
  component: SignUp,
});

function SignUp() {
  return (
    <div className="w-full flex flex-col justify-center items-center mt-8 gap-y-4">
      <Header />
      <SignUpCard />
    </div>
  );
}
