import { useForm } from "react-hook-form";
import { signUpSchema, type SignUpType } from "@/schema/signup";
import { zodResolver } from "@hookform/resolvers/zod";

import { Button } from "@/components/primitives/button";

import { Input } from "@/components/primitives/input";
import { Label } from "@/components/primitives/label";

export default function SignUpForm() {
  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm<SignUpType>({
    resolver: zodResolver(signUpSchema),
  });

  return (
    <form
      className="grid gap-4"
      onSubmit={handleSubmit((val) => console.log(val))}
    >
      <div className="grid grid-cols-2 gap-x-4 gap-y-2">
        <div className="grid gap-2">
          <Label className="text-secondary" htmlFor="firstName">
            First name
          </Label>
          <Input
            disabled={isSubmitting}
            {...register("firstName")}
            className="border-secondary focus:text-primary focus-visible:border-none focus-visible:text-primary focus-visible:ring-offset-0"
            placeholder="John"
            type="text"
          />
        </div>
        <div className="grid gap-2">
          <Label className="text-secondary" htmlFor="lastName">
            Last name
          </Label>
          <Input
            disabled={isSubmitting}
            {...register("lastName")}
            className="border-secondary focus:text-primary focus-visible:border-none focus-visible:text-primary focus-visible:ring-offset-0"
            placeholder="Doe"
            type="text"
          />
        </div>
        {(errors.firstName || errors.lastName) && (
          <span className="col-start-1 col-end-3 text-xs text-red-500">
            {errors.firstName?.message ?? errors.lastName?.message}
          </span>
        )}
      </div>
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
          {...register("password")}
          className="border-secondary focus:text-primary focus-visible:border-none focus-visible:text-primary focus-visible:ring-offset-0"
          type="password"
          placeholder="********"
        />
        {errors.password && (
          <span className="text-xs text-red-500">
            {errors.password.message}
          </span>
        )}
      </div>
      <Button disabled={isSubmitting} type="submit" className="w-full">
        Create an account
      </Button>
    </form>
  );
}
