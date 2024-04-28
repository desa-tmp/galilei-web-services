import { Layout } from "@/lib/types";

export default function AuthLayout({ children }: Layout) {
  return (
    <div className="flex size-full flex-col items-center justify-center px-12 py-9">
      {children}
    </div>
  );
}
