import { Link } from "@tanstack/react-router";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

import LoginForm from "./LoginForm";

export default function LoginCard() {
  return (
    <Card className="w-full max-w-sm border-none shadow-none">
      <CardHeader className="flex flex-col">
        <CardTitle className="text-2xl text-secondary">Login</CardTitle>
        <CardDescription className="text-secondary">
          Enter your credentials below to login to your account.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <LoginForm />
      </CardContent>
      <CardFooter>
        <div className="mx-auto mt-4 text-center text-sm text-secondary">
          Don&apos;t have an account?{" "}
          <Link
            to="/sign-up"
            className="text-secondary underline underline-offset-2 outline-none transition-all duration-500 hover:text-primary"
          >
            Sign up
          </Link>
        </div>
      </CardFooter>
    </Card>
  );
}
