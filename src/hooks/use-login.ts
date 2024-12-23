import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";

import { loginSchema, LoginType } from "@/schema/login";
import { IpcUserType } from "@/types/ipc";

export const useLogin = () => {
  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm<LoginType>({
    resolver: zodResolver(loginSchema),
  });

  const onSubmit = async (val: LoginType) => {
    const user = await invoke<IpcUserType>("login", val);

    console.log({ user });
  };

  return {
    handleSubmit: handleSubmit(onSubmit),
    register,
    errors,
    isSubmitting,
  };
};
