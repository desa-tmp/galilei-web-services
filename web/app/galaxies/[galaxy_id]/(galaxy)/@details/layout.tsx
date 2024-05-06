import { Layout } from "@/lib/types";

export default function DetailsLayout({ children }: Layout) {
  return <aside className="px-4 py-6">{children}</aside>;
}
