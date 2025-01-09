import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useNavigate } from "@tanstack/react-router";

import { useAuth } from "@/hooks/use-auth";
import { loginSchema, LoginType } from "@/schema/login";
import { useToast } from "./use-toast";
import { createTauRPCProxy, ErrorPayload } from "@/types/taurpc";

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
    const taurpc = await createTauRPCProxy();
    const user = await taurpc.auth
      .login(val.email, val.password)
      .catch((err: ErrorPayload) => {
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
      title: "Login Successfull!",
      description: "You have been successfully Logged in!",
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
