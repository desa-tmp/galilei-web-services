import { Layout, Page } from "@/lib/types";
import Link from "next/link";

export default function DetailsLayout({
  children,
  params,
}: Layout & Page<{ galaxy_id: string }>) {
  return (
    <aside className="size-full px-4 pb-6 pt-3">
      <div>
        <Link href={`/galaxies/${params.galaxy_id}`}>{"<"} close</Link>
      </div>
      <div>{children}</div>
    </aside>
  );
}
