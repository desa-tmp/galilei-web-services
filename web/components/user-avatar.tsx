import { api } from "@/lib/api";
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
import ActionBtn from "./action-btn";
import { ApiError } from "api-client";
import { logout } from "@/lib/actions";

export default async function UserAvatar() {
  const { data: user, error } = await api.GET("/users/me");

  if (error) {
    throw new ApiError(error);
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Avatar className="transition-shadow hover:ring-2" asChild>
          <button>
            <AvatarFallback className="uppercase">
              {user.name[0]}
            </AvatarFallback>
          </button>
        </Avatar>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56">
        <DropdownMenuLabel className="text-center text-lg">
          <span>{user.name}</span>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem
          className="text-red-500 hover:bg-accent hover:text-red-500 focus:text-red-500"
          asChild
        >
          <ActionBtn action={logout} className="flex w-full items-center gap-2">
            <DoorOpen className="size-4" />
            <span>log out</span>
          </ActionBtn>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
