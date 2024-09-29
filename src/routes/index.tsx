import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
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
