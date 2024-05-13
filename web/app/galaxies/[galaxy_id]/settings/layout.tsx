import { Layout } from "@/lib/types";

export default function GalaxySettingsLayout({ children }: Layout) {
  return (
    <main className="flex size-full flex-col items-center justify-center gap-6">
      {children}
    </main>
  );
}
