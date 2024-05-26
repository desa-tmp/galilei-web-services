"use client";

import { DoorOpen, Loader2 } from "lucide-react";
import { DropdownMenuItem } from "./ui/dropdown-menu";
import { MouseEvent, useTransition } from "react";
import { logout } from "@/lib/actions";
import { cn } from "@/lib/utils";

export default function LogoutMenuItem() {
  const [isPending, startTransition] = useTransition();

  async function handleLogout(e: MouseEvent<HTMLDivElement>) {
    e.preventDefault();
    startTransition(async () => {
      await logout();
    });
  }

  const Icon = isPending ? Loader2 : DoorOpen;

  return (
    <DropdownMenuItem
      onClick={handleLogout}
      className="cursor-pointer text-red-500 hover:bg-accent hover:text-red-500 focus:text-red-500"
    >
      <Icon className={cn("mr-2 size-4", isPending && "animate-spin")} />
      <span>log out</span>
    </DropdownMenuItem>
  );
}
