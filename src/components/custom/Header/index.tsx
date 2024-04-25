import logo from "@/assets/lock_monochrome.png";

export default function Header() {
  return (
    <div className="flex items-center justify-center gap-x-3">
      <img
        src={logo}
        alt="Logo"
        className="aspect-square h-16 w-16 self-center dark:invert"
      />
      <h1 className="mb-1 self-end font-serif text-4xl">Cypher</h1>
    </div>
  );
}
