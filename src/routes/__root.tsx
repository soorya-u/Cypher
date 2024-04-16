import { createRootRoute, Outlet } from "@tanstack/react-router";

import Navigator from "@/components/custom/Navigator";
import TanStackRouterDevtools from "@/components/custom/TanstackDevTools";

export const Route = createRootRoute({
  component: () => (
    <>
      <Navigator />
      <Outlet />
      <TanStackRouterDevtools />
    </>
  ),
});
