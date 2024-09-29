import {
  RouterProvider as TanstackRouterProvider,
  createRouter,
} from "@tanstack/react-router";
import { routeTree } from "@/routeTree.gen";

const router = createRouter({ routeTree });
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
export default function RouterProvider() {
  return <TanstackRouterProvider router={router} />;
}
