import logo from "@/assets/lock_monochrome.png";

export default function Header() {
  return (
    <div className="flex justify-center items-center">
      <img
        src={logo}
        alt="Logo"
        className="dark:invert self-center h-20 w-20 aspect-square"
      />
      <h1 className="font-serif text-5xl self-end">Cypher</h1>
    </div>
  );
}
