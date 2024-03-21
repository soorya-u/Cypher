import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  async function greet() {
    return await invoke("greet", { name });
  }

  return <></>;
}

export default App;
