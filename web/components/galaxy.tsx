import { fetchApi } from "@/lib/api";
import { Galaxy as GalaxyType, Planet, Star } from "@/lib/schema";
import Link from "next/link";

interface GalaxyProps {
  galaxy_id: string;
}

export default async function Galaxy({ galaxy_id }: GalaxyProps) {
  const { galaxy, stars, planets } = (await (
    await fetchApi(`/galaxies/${galaxy_id}`, { tags: ["galaxy"] })
  ).json()) as {
    galaxy: GalaxyType;
    stars: Star[];
    planets: Planet[];
  };

  return (
    <div className="flex flex-col gap-4">
      <h1>{galaxy.name}</h1>
      <div>
        <h1>Stars</h1>
        <ul>
          {stars.map(({ id, name }) => (
            <li key={id}>
              <Link href={`/galaxies/${galaxy_id}/stars/${id}`}>{name}</Link>
            </li>
          ))}
        </ul>
      </div>
      <div>
        <h1>Planets</h1>
        <ul>
          {planets.map(({ id, name }) => (
            <li key={id}>
              <Link href={`/galaxies/${galaxy_id}/planets/${id}`}>{name}</Link>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
