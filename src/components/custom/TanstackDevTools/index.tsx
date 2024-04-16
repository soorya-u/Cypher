import { lazy, Suspense } from "react";

const DevTool =
  process.env.NODE_ENV === "production"
    ? () => null
    : lazy(() =>
        import("@tanstack/router-devtools").then((res) => ({
          default: res.TanStackRouterDevtools,
        }))
      );

export default function TanStackRouterDevtools() {
  return (
    <Suspense>
      <DevTool />
    </Suspense>
  );
}
