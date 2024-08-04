import { useForm } from "react-hook-form";
import { loginSchema, type LoginType } from "@/schema/login";
import { zodResolver } from "@hookform/resolvers/zod";

import { Input } from "@/components/primitives/input";
import { Label } from "@/components/primitives/label";
import { Button } from "@/components/primitives/button";

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
        <Label className="text-secondary" htmlFor="email">
          Email
        </Label>
        <Input
          disabled={isSubmitting}
          {...register("email")}
          className="border-secondary focus:text-primary focus-visible:border-none focus-visible:text-primary focus-visible:ring-offset-0"
          placeholder="johndoe@example.com"
          type="text"
        />
        {errors.email && (
          <span className="text-xs text-red-500">{errors.email.message}</span>
        )}
      </div>
      <div className="grid gap-2">
        <Label className="text-secondary" htmlFor="password">
          Password
        </Label>
        <Input
          disabled={isSubmitting}
          aria-disabled={isSubmitting}
          {...register("password")}
          type="password"
          placeholder="********"
          className="border-secondary focus:text-primary focus-visible:border-none focus-visible:text-primary focus-visible:ring-offset-0"
        />
        {errors.password && (
          <span className="text-xs text-red-500">
            {errors.password.message}
          </span>
        )}
      </div>
      <Button className="w-full bg-primary">Sign in</Button>
    </form>
  );
}
