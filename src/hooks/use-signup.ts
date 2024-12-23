import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";

import { signUpSchema, SignUpType } from "@/schema/signup";
import { IpcSignUpType, IpcUserType } from "@/types/ipc";

export const useSignUp = () => {
  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm<SignUpType>({
    resolver: zodResolver(signUpSchema),
  });

  const onSubmit = async (val: SignUpType) => {
    const payload: IpcSignUpType = {
      email: val.email,
      password: val.password,
      fullName: `${val.firstName} ${val.lastName}`,
    };

    const user = await invoke<IpcUserType>("sign_up", payload);

    console.log({ user });
  };

  return {
    handleSubmit: handleSubmit(onSubmit),
    register,
    errors,
    isSubmitting,
  };
};
