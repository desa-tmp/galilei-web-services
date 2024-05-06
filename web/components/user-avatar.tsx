import { fetchApi } from "@/lib/api";
import { User } from "@/lib/schema";
import { Avatar, AvatarFallback } from "./ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";
import { DoorOpen } from "lucide-react";
import ActionClick from "./action-click";
import { redirect } from "next/navigation";

export default async function UserAvatar() {
  const user = (await (await fetchApi("/users/me")).json()) as User;

  async function logout() {
    "use server";
    await fetchApi("/auth/logout", {
      method: "DELETE",
    });

    redirect("/login");
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Avatar className="cursor-pointer transition-shadow hover:ring-2">
          <AvatarFallback className="uppercase">{user.name[0]}</AvatarFallback>
        </Avatar>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56">
        <DropdownMenuLabel className="text-center text-lg">
          <span>{user.name}</span>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem className="cursor-pointer text-red-500 hover:bg-accent">
          <ActionClick action={logout} className="flex items-center gap-2">
            <DoorOpen className="size-4" />
            <span>log out</span>
          </ActionClick>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
