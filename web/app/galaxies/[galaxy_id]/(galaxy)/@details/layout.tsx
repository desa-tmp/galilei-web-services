import { Layout } from "@/lib/types";

export default function DetailsLayout({ children }: Layout) {
  return <div className="px-4 py-6">{children}</div>;
}
