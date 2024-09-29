import ReactDOM from "react-dom/client";

import Providers from "@/providers";
import "@/styles.css";

const rootElement = document.getElementById("app")!;
const root = ReactDOM.createRoot(rootElement);
root.render(<Providers />);
