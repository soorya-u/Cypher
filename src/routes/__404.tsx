import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/__404")({
  component: About,
});

function About() {
  return <div className="p-2">Not Found</div>;
}
