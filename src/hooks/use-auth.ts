import { useEffect } from "react";
import { create } from "zustand";
import { useNavigate } from "@tanstack/react-router";

import { createTauRPCProxy, ErrorPayload, IpcUser } from "@/types/taurpc";

type AuthState = {
  email: string;
  name: string;
};

type AuthActions = {
  setSession: (cred: IpcUser) => void;
  clearSession: () => void;
};

export const useAuth = create<AuthState & AuthActions>((set) => ({
  name: "",
  email: "",
  setSession: (cred) =>
    set(() => ({ email: cred.email, name: cred.full_name })),
  clearSession: () => set({ name: "", email: "" }),
}));

export const useSession = () => {
  const navigate = useNavigate();
  const { setSession, email, name } = useAuth();
  useEffect(() => {
    if (!email || !name) {
      const taurpc = createTauRPCProxy();
      taurpc.auth
        .getSession()
        .then((u) => setSession(u))
        .catch((_: ErrorPayload) => navigate({ to: "/login" }));
    }
  }, []);
};

export const useLogout = () => {
  const navigate = useNavigate();
  const { clearSession } = useAuth();
  const logout = async () => {
    const taurpc = await createTauRPCProxy();
    await taurpc.auth.logout().then(() => {
      clearSession();
      navigate({ to: "/login", replace: true, startTransition: true });
    });
  };

  return logout;
};
