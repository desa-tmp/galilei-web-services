import { Page } from "@/lib/types";

type StarPage = Page<{ galaxy_id: string; star_id: string }>;

export default function Star({ params: { galaxy_id, star_id } }: StarPage) {
  return (
    <h1>
      Star id: {star_id} in galaxy id: {galaxy_id}
    </h1>
  );
}
