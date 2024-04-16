import { Link } from "@tanstack/react-router";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export default function LoginCard() {
  return (
    <Card className="w-full max-w-sm border-none">
      <CardHeader className="flex flex-col">
        <CardTitle className="text-2xl">Login</CardTitle>
        <CardDescription className="text-[#b4a69e]">
          Enter your credentials below to login to your account.
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <div className="grid gap-2">
          <Label htmlFor="username">Username</Label>
          <Input
            id="username"
            type="text"
            placeholder="john-doe"
            className="border-[#b4a69e] focus-visible:border-none focus-visible:ring-offset-0"
            required
          />
        </div>
        <div className="grid gap-2">
          <Label htmlFor="password">Password</Label>
          <Input
            id="password"
            type="password"
            className="border-[#b4a69e] focus-visible:border-none focus-visible:ring-offset-0"
            required
          />
        </div>
        <div className="mt-4 text-center text-sm">
          Don&apos;t have an account?{" "}
          <Link
            to="/signup"
            className="underline underline-offset-2 outline-none"
          >
            Sign up
          </Link>
        </div>
      </CardContent>
      <CardFooter>
        <Button className="w-full bg-primary">Sign in</Button>
      </CardFooter>
    </Card>
  );
}
