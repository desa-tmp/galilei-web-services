import { fetchApi } from "@/lib/api";
import { Galaxy } from "@/lib/schema";
import Link from "next/link";

export default async function Galaxies() {
  const galaxies = (await (await fetchApi("/galaxies")).json()) as Galaxy[];

  return (
    <div className="flex flex-col gap-4">
      {galaxies.map(({ id, name }) => (
        <Link key={id} href={`/galaxies/${id}`}>
          {name}
        </Link>
      ))}
    </div>
  );
}
