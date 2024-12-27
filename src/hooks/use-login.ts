import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "@tanstack/react-router";

import { useAuth } from "@/hooks/use-auth";
import { loginSchema, LoginType } from "@/schema/login";
import { IpcUserType, InvokableFunctions, IpcErrorPayload } from "@/types/ipc";
import { useToast } from "./use-toast";

export const useLogin = () => {
  const { setSession } = useAuth();
  const navigate = useNavigate();
  const { toast } = useToast();
  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm<LoginType>({
    resolver: zodResolver(loginSchema),
  });

  const onSubmit = async (val: LoginType) => {
    const user = await invoke<IpcUserType>(InvokableFunctions.Login, val).catch(
      (err: IpcErrorPayload) => {
        console.log({ failed: err.details, error: err.error });
        toast({
          title: "Failed to Login!",
          description: err?.message || "Something went wrong",
        });
      },
    );
    if (!user)
      return toast({
        title: "Something went wrong!",
        description: "Unable to get User",
      });
    setSession(user);
    navigate({ to: "/", startTransition: true, replace: true });
  };

  return {
    handleSubmit: handleSubmit(onSubmit),
    register,
    errors,
    isSubmitting,
  };
};
