import { useEffect } from "react";
import { useNavigate } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";

import { IpcUserType, IpcErrorPayload } from "@/types/ipc";

export const useSession = () => {
  const navigate = useNavigate();

  useEffect(() => {
    (async () =>
      await invoke<IpcUserType>("get_session")
        .then((u) => {
          console.log({ u });
          // Add to State Manager
          return;
        })
        .catch((err: IpcErrorPayload) => {
          console.log({ err });
          navigate({ to: "/login" });
        }))();
  }, []);
};
