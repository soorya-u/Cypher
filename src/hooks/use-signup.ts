import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "@tanstack/react-router";

import { signUpSchema, SignUpType } from "@/schema/signup";
import {
  IpcSignUpType,
  IpcUserType,
  InvokableFunctions,
  IpcErrorPayload,
} from "@/types/ipc";
import { useAuth } from "./use-auth";
import { useToast } from "./use-toast";

export const useSignUp = () => {
  const { setSession } = useAuth();
  const navigate = useNavigate();
  const { toast } = useToast();
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

    const user = await invoke<IpcUserType>(
      InvokableFunctions.SignUp,
      payload,
    ).catch((err: IpcErrorPayload) => {
      console.log({ failed: err.details, error: err.error });
      toast({
        title: "Failed to Login!",
        description: err?.message || "Something went wrong",
        variant: "destructive",
      });
    });
    if (!user)
      return toast({
        title: "Something went wrong!",
        description: "Unable to get User",
        variant: "destructive",
      });
    toast({
      title: "Sign Up Successfull!",
      description: "Your account has been successfully Created!",
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
