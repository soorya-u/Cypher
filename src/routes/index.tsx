import { createFileRoute } from "@tanstack/react-router";

import { useAuth, useSession } from "@/hooks/use-auth";

export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
  useAuth.subscribe((state) => console.log(state));
  useSession();

  return <div className="p-2"></div>;
}
