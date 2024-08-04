import { Link } from "@tanstack/react-router";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

import SignUpForm from "./SignUpForm";

export default function SignUpCard() {
  return (
    <Card className="mx-auto max-w-sm border-none shadow-none">
      <CardHeader>
        <CardTitle className="text-2xl text-secondary">Sign Up</CardTitle>
        <CardDescription className="text-secondary">
          Enter your information to create an account
        </CardDescription>
      </CardHeader>
      <CardContent>
        <SignUpForm />
        <div className="mt-4 text-center text-sm text-secondary">
          Already have an account?{" "}
          <Link
            to="/login"
            className="text-secondary underline transition-all duration-500 hover:text-primary"
          >
            Login
          </Link>
        </div>
      </CardContent>
    </Card>
  );
}
