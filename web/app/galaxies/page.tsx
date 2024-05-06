import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { fetchApi } from "@/lib/api";
import { Galaxy } from "@/lib/schema";
import { Orbit, Plus } from "lucide-react";
import Link from "next/link";

export default async function Galaxies() {
  const galaxies = (await (await fetchApi("/galaxies")).json()) as Galaxy[];

  return (
    <div className="flex size-full flex-col gap-6 px-12 pb-9 pt-12">
      <header className="flex flex-col gap-6">
        <h1 className="text-2xl font-bold">Your Galaxies</h1>
        <Button className="flex gap-2 self-end" asChild>
          <Link href="/galaxies/new">
            <Plus className="size-4" />
            <span>New Galaxy</span>
          </Link>
        </Button>
      </header>
      <ScrollArea className="flex-1" type="auto">
        <ul className="grid grid-cols-fluid gap-x-4 gap-y-6 p-4">
          {galaxies.map(({ id, name }) => (
            <li
              key={id}
              className="w-full cursor-pointer overflow-hidden rounded-md border border-border transition-colors hover:bg-secondary/80"
            >
              <Link
                href={`/galaxies/${id}`}
                className="flex size-full items-center gap-2 px-6 py-4"
              >
                <Orbit />
                <span>{name}</span>
              </Link>
            </li>
          ))}
        </ul>
      </ScrollArea>
    </div>
  );
}
