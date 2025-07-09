import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import { toast } from "sonner";
import Cookies from "js-cookie";

import { Env } from "@/lib/env";
import { useAuthStore } from "@/store/auth-store";
import type { SessionClaim } from "@/utils/auth/types";

const verifyAuthenticationCall = (): Promise<SessionClaim> =>
  axios
    .get(`${Env.server_url}/api/auth/verify`, {
      withCredentials: true,
    })
    .then((res) => res.data.data);

export const useVerifyAuthentication = () => {
  const setSession = useAuthStore((s) => s.setSession);
  const setError = useAuthStore((s) => s.setError);
  const setLoading = useAuthStore((s) => s.setLoading);
  const clearError = useAuthStore((s) => s.clearError);
  const reset = useAuthStore((s) => s.reset);

  const query = useQuery({
    queryKey: ["api", "auth", "verify"],
    queryFn: verifyAuthenticationCall,
    retry: 1,
    refetchOnMount: "always",
    refetchInterval: 5 * 60 * 1000,
    refetchOnWindowFocus: true,
    refetchIntervalInBackground: true,
    refetchOnReconnect: true,
  });

  useEffect(() => {
    setLoading(query.isLoading);
  }, [query.isLoading, setLoading]);

  useEffect(() => {
    if (query.isSuccess && query.data) {
      setSession(query.data);
      clearError();
    }
  }, [query.isSuccess, query.data, setSession, clearError]);

  useEffect(() => {
    if (query.isError) {
      reset();
      setError("Authentication failed or session expired");

      const popupShown = Cookies.get("login_popup_shown");

      if (!popupShown) {
        toast.warning("Access denied. Suit up and log in, captain.", {
          position: "top-center",
          duration: 3000,
          richColors: true,
          closeButton: true,
        });
        Cookies.set("login_popup_shown", "true", {
          expires: 1 / 288,
        });
      }
    }
  }, [query.isError, reset, setError]);

  return {
    user: query.data,
    ...query,
  };
};
