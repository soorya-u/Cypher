import { useLogin } from "@/hooks/use-login";

import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";

export default function LoginForm() {
  const { handleSubmit, register, errors, isSubmitting } = useLogin();
  return (
    <form onSubmit={handleSubmit} className="grid gap-4">
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
      <Button className="w-full bg-primary text-black">Sign in</Button>
    </form>
  );
}
