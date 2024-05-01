import logo from "@/assets/image/lock_monochrome.png";

export default function Header() {
  return (
    <div className="flex items-center justify-center gap-x-3">
      <img
        src={logo}
        alt="Logo"
        className="aspect-square h-16 w-16 self-center dark:invert"
      />
      <h1 className="self-end font-['GlitchInside'] text-6xl">Cypher</h1>
    </div>
  );
}
