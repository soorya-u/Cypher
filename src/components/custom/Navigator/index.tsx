import { Link, useRouterState } from "@tanstack/react-router";

export default function Navigator() {
  const router = useRouterState();
  const a = router.location.pathname;

  return (
    <div className="p-2 flex gap-2">
      { true && a !== "/signup" && a !== "/login" && (
        <>
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
        </>
      )}
    </div>
  );
}
