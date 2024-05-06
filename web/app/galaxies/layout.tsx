import { Button } from "@/components/ui/button";
import UserAvatar from "@/components/UserAvatar";
import { Layout } from "@/lib/types";
import { Orbit } from "lucide-react";
import Link from "next/link";

export default function GalaxiesLayout({ children }: Layout) {
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
          <UserAvatar />
        </div>
      </header>
      <div className="flex-1 overflow-hidden">{children}</div>
    </div>
  );
}
