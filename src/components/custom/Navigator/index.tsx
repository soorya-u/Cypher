import { Link, useMatchRoute } from "@tanstack/react-router";

export default function Navigator() {
  const matchRoute = useMatchRoute();
  const authRoute =
    matchRoute({ to: "/signup" }) || matchRoute({ to: "/login" });

  return (
    !authRoute && (
      <div className="flex gap-2 p-2">
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
    )
  );
}
