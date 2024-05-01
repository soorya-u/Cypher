import { PropsWithChildren } from "react";

import { Link, useMatchRoute } from "@tanstack/react-router";

import { cn } from "@/utils/cn";

type NavigatorProps = {
  className?: string;
} & PropsWithChildren;

export default function Navigator(props: NavigatorProps) {
  const matchRoute = useMatchRoute();
  const authRoute =
    matchRoute({ to: "/signup" }) || matchRoute({ to: "/login" });

  return (
    !authRoute && (
      <>
        <div
          className={cn(
            props.className,
            "flex h-screen flex-col items-center justify-start gap-2 p-2",
          )}
        >
          <Link to="/" className="[&.active]:font-bold">
            Home
          </Link>{" "}
          <Link to="/about" className="[&.active]:font-bold">
            About
          </Link>
          <Link to="/login" className="[&.active]:font-bold">
            Login
          </Link>
          <Link to="/signup" className="[&.active]:font-bold">
            Signup
          </Link>
        </div>
        {props.children}
      </>
    )
  );
}
