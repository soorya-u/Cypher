import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  async function greet() {
    return await invoke("greet", { name });
  }

  return <h1 className="text-red-600">Hello World</h1>;
}

export default App;
