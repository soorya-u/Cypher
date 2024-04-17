import { useForm } from "react-hook-form";
import { loginSchema, type LoginType } from "@/schema/login";
import { zodResolver } from "@hookform/resolvers/zod";

import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";

export default function LoginForm() {
  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm<LoginType>({
    resolver: zodResolver(loginSchema),
  });
  return (
    <form
      onSubmit={handleSubmit((val) => console.log(val))}
      className="grid gap-4"
    >
      <div className="grid gap-2">
        <Label htmlFor="email">Email</Label>
        <Input
          disabled={isSubmitting}
          {...register("email")}
          className="border-[#b4a69e] focus-visible:border-none focus-visible:ring-offset-0"
          placeholder="johndoe@example.com"
          type="text"
        />
        {errors.email && (
          <span className="text-red-500 text-xs">{errors.email.message}</span>
        )}
      </div>
      <div className="grid gap-2">
        <Label htmlFor="password">Password</Label>
        <Input
          disabled={isSubmitting}
          aria-disabled={isSubmitting}
          {...register("password")}
          type="password"
          className="border-[#b4a69e] focus-visible:border-none focus-visible:ring-offset-0"
        />
        {errors.password && (
          <span className="text-red-500 text-xs">
            {errors.password.message}
          </span>
        )}
      </div>
      <Button className="w-full bg-primary">Sign in</Button>
    </form>
  );
}
