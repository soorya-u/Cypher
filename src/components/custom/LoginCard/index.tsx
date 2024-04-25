import { Link } from "@tanstack/react-router";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/primitives/card";

import LoginForm from "./LoginForm";

export default function LoginCard() {
  return (
    <Card className="w-full max-w-sm border-none shadow-none">
      <CardHeader className="flex flex-col">
        <CardTitle className="text-2xl">Login</CardTitle>
        <CardDescription className="text-[#b4a69e]">
          Enter your credentials below to login to your account.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <LoginForm />
      </CardContent>
      <CardFooter>
        <div className="mx-auto mt-4 text-center text-sm">
          Don&apos;t have an account?{" "}
          <Link
            to="/signup"
            className="underline underline-offset-2 outline-none"
          >
            Sign up
          </Link>
        </div>
      </CardFooter>
    </Card>
  );
}
