import { createRootRoute, Outlet } from "@tanstack/react-router";

import Navigator from "@/components/custom/Navigator";
import TanStackRouterDevtools from "@/components/custom/TanstackDevTools";

export const Route = createRootRoute({
  component: () => (
    <div className="flex h-full w-full items-start justify-start">
      <Navigator className="w-1/4">
        <div className="h-full w-[1px] bg-gray-400" />
      </Navigator>
      <div className="flex-1 h-full">
        <Outlet />
      </div>
      <TanStackRouterDevtools />
    </div>
  ),
});
