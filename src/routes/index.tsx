import { createFileRoute } from "@tanstack/react-router";

import { useSession } from "@/hooks/use-session";

export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
  useSession();

  return <div className="p-2"></div>;
}
