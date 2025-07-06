// components/RouteGuard.tsx
import { useRouter } from "@tanstack/react-router";
import { useEffect } from "react";
import { useVerifyAuthentication } from "@/hooks/use-verify-auth";
import { toast } from "sonner";

export const RouteGuard = ({ children }: { children: React.ReactNode }) => {
  const router = useRouter();

  const { user, isLoading } = useVerifyAuthentication();

  useEffect(() => {
    if (isLoading) return;

    const path = router.state.location.pathname;
    const isPublicPath = path === "/";

    if (!user && !isPublicPath) {
      router.navigate({ to: "/", resetScroll: true });
      toast.error("Please login to access other pages!");
    }
  }, [user, isLoading, router.state.location]);

  return <>{children}</>;
};
