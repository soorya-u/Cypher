import { PropsWithChildren, useState } from "react";
import { motion } from "framer-motion";
import { Link } from "@tanstack/react-router";

import { IconHome2, IconSettings, IconLogout } from "@tabler/icons-react";

import {
  Sidebar as SidebarUI,
  SidebarBody,
  SidebarLink,
  SidebarClickable,
  Links,
  Clickable,
} from "@/components/ui/sidebar";

import { cn } from "@/utils/cn";

import { ThemeToggler } from "../ThemeToggler";

import logo from "@/assets/image/logo-thick.png";
import { useAuth, useLogout } from "@/hooks/use-auth";

export function Sidebar({ children }: PropsWithChildren) {
  const [open, setOpen] = useState(false);
  const logout = useLogout();
  const { name } = useAuth();

  return (
    <div
      className={cn(
        "flex w-full flex-1 flex-col overflow-hidden rounded-md border border-neutral-200 bg-gray-100 dark:border-neutral-700 dark:bg-neutral-800 md:flex-row",
        "h-screen",
      )}
    >
      <SidebarUI open={open} setOpen={setOpen}>
        <SidebarBody className="justify-between gap-10">
          <div className="flex flex-1 flex-col overflow-y-auto overflow-x-hidden">
            <Logo isOpen={open} />
            <div className="mt-8 flex flex-col gap-2">
              {links.map((link, idx) => {
                if (link.type === "clickable" && link.label === "Logout")
                  link.handleClick = logout;
                return link.type === "link" ? (
                  <SidebarLink key={idx} link={link} />
                ) : (
                  <SidebarClickable key={idx} link={link} />
                );
              })}
              <ThemeToggler />
            </div>
          </div>
          <div>
            <SidebarLink
              link={{
                label: name,
                href: "#",
                icon: (
                  <img
                    src="https://www.soorya-u.dev/apple-touch-icon.png"
                    className="h-7 w-7 flex-shrink-0 rounded-full"
                    alt="Avatar"
                  />
                ),
              }}
            />
          </div>
        </SidebarBody>
      </SidebarUI>
      <div className="flex flex-1">
        <div className="flex h-full w-full flex-1 flex-col gap-2 rounded-tl-2xl border border-neutral-200 bg-white p-4 dark:border-neutral-700 dark:bg-neutral-900">
          {children}
        </div>
      </div>
    </div>
  );
}

export const Logo = ({ isOpen }: { isOpen: boolean }) => {
  return (
    <Link
      href="/"
      className="relative z-20 mt-2 flex items-center space-x-2 py-1 text-sm font-normal text-black"
    >
      <img src={logo} alt="Logo" className="aspect-square size-7 self-center" />
      {isOpen && (
        <motion.span
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="whitespace-pre font-Iceberg text-xl font-bold tracking-widest text-primary"
        >
          Cypher
        </motion.span>
      )}
    </Link>
  );
};

const links: (
  | (Links & { type: "link" })
  | (Clickable & { type: "clickable" })
)[] = [
  {
    type: "link",
    label: "Dashboard",
    href: "/",
    icon: (
      <IconHome2 className="h-5 w-5 flex-shrink-0 text-neutral-700 dark:text-neutral-200" />
    ),
  },
  {
    type: "link",
    label: "Settings",
    href: "/",
    icon: (
      <IconSettings className="h-5 w-5 flex-shrink-0 text-neutral-700 dark:text-neutral-200" />
    ),
  },
  {
    type: "clickable",
    label: "Logout",
    handleClick: async () => {},
    icon: (
      <IconLogout className="h-5 w-5 flex-shrink-0 text-neutral-700 dark:text-neutral-200" />
    ),
  },
];
