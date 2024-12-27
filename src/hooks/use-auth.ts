import { useEffect } from "react";
import { create } from "zustand";
import { useNavigate } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";

import { IpcUserType, IpcErrorPayload, InvokableFunctions } from "@/types/ipc";

type AuthState = {
  email: string;
  name: string;
};

type AuthActions = {
  setSession: (cred: IpcUserType) => void;
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
    if (!email || !name)
      invoke<IpcUserType>(InvokableFunctions.GetSession)
        .then((u) => setSession(u))
        .catch((_: IpcErrorPayload) => navigate({ to: "/login" }));
  }, []);
};

export const useLogout = () => {
  const navigate = useNavigate();
  const { clearSession } = useAuth();
  const logout = async () => {
    await invoke(InvokableFunctions.Logout).then(() => {
      clearSession();
      navigate({ to: "/login", replace: true, startTransition: true });
    });
  };

  return logout;
};
