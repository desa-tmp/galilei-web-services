import { Layout } from "@/lib/types";

export default function GalaxySettingsLayout({ children }: Layout) {
  return (
    <main className="flex size-full items-center justify-center">
      {children}
    </main>
  );
}
