import { useEffect } from "react";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";

export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
  const navigate = useNavigate();

  useEffect(() => {
    (async () =>
      await invoke<IPCUserType>("get_session")
        .then((_) => {
          return;
        })
        .catch((err: IPCError) => {
          console.log({ err });
          navigate({ to: "/login" });
        }))();
  }, []);

  return (
    <div className="p-2">
      <button
        onClick={async () => {
          const a = await invoke("db_init");
          console.log({ a });
        }}
      >
        Init DB
      </button>
    </div>
  );
}
