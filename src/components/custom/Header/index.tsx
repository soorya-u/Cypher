import { IndependentThemeToggler } from "@/components/custom/ThemeToggler";
import logo from "@/assets/image/logo-thick.png";

export default function Header() {
  return (
    <div className="relative flex items-center justify-center gap-x-3">
      <IndependentThemeToggler />
      <img
        src={logo}
        alt="Logo"
        className="aspect-square h-16 w-16 self-center"
      />
      <h1 className="font-Iceberg self-end text-6xl font-bold text-primary">
        Cypher
      </h1>
    </div>
  );
}
