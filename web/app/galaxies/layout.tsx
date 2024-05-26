import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Layout } from "@/lib/types";
import { Orbit } from "lucide-react";
import Link from "next/link";
import { api } from "@/lib/api";
import { ApiError } from "api-client";
import LogoutMenuItem from "@/components/logout-menu-item";

export default async function GalaxiesLayout({ children }: Layout) {
  const { data: user, error } = await api.GET("/users/me");

  if (error) {
    throw new ApiError(error);
  }

  return (
    <div className="flex size-full flex-col">
      <header className="flex w-full items-center justify-between gap-6 border-b-2 border-border px-6 py-2">
        <Link href="/galaxies" className="font-bold">
          GWS
        </Link>
        <div className="flex items-center gap-4">
          <Button className="flex items-center gap-2" asChild>
            <Link href="/galaxies/new">
              <span>new</span>
              <Orbit className="size-4" />
            </Link>
          </Button>
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
              <LogoutMenuItem />
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </header>
      <div className="flex-1 overflow-hidden">{children}</div>
    </div>
  );
}
