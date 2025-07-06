import { Outlet, createRootRouteWithContext } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

import TanStackQueryLayout from "../integrations/tanstack-query/layout.tsx";

import type { QueryClient } from "@tanstack/react-query";
import { RouteGuard } from "@/lib/guards/auth-guards.tsx";
import { Toaster } from "@/components/ui/sonner.tsx";

interface MyRouterContext {
  queryClient: QueryClient;
}

export const Route = createRootRouteWithContext<MyRouterContext>()({
  component: () => (
    <>
      <RouteGuard>
        <Outlet />
        <Toaster />
      </RouteGuard>
    </>
  ),
});
