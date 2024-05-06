import { Layout } from "@/lib/types";

export default function NewGalaxyLayout({ children }: Layout) {
  return (
    <main className="flex size-full items-center justify-center">
      {children}
    </main>
  );
}
