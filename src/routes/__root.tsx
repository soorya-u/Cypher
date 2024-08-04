import { createRootRoute, Outlet, useMatchRoute } from "@tanstack/react-router";

import TanStackRouterDevtools from "@/components/custom/TanstackDevTools";
import { Sidebar } from "@/components/custom/Navigator";

const RootPage = () => {
  const matchRoute = useMatchRoute();
  const authRoute =
    matchRoute({ to: "/sign-up" }) || matchRoute({ to: "/login" });

  return !authRoute ? (
    <Sidebar>
      <Outlet />
      <TanStackRouterDevtools />
    </Sidebar>
  ) : (
    <>
      <Outlet />
      <TanStackRouterDevtools />
    </>
  );
};

export const Route = createRootRoute({
  component: RootPage,
});
