import logo from "@/assets/lock_monochrome.png";

export default function Header() {
  return (
    <div className="flex justify-center items-center gap-x-3">
      <img
        src={logo}
        alt="Logo"
        className="dark:invert self-center h-16 w-16 aspect-square"
      />
      <h1 className="font-serif text-4xl self-end mb-1">Cypher</h1>
    </div>
  );
}
