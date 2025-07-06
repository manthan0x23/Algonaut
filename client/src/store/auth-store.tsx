import { create } from "zustand";
import type { SessionClaim } from "@/utils/auth/types";

export interface AuthState {
  session: SessionClaim | null;
  isLoading: boolean;
  isError: boolean;
  error: string;
  setSession: (session: SessionClaim) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  clearError: () => void;
  reset: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  session: null,
  isLoading: false,
  isError: false,
  error: "",

  setSession: (session) =>
    set({
      session,
      isLoading: false,
      isError: false,
      error: "",
    }),

  setLoading: (isLoading) => set({ isLoading }),

  setError: (error) =>
    set({
      isError: true,
      error,
      isLoading: false,
    }),

  clearError: () =>
    set({
      isError: false,
      error: "",
    }),

  reset: () =>
    set({
      session: null,
      isLoading: false,
      isError: false,
      error: "",
    }),
}));
