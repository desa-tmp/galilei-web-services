import Link from "next/link";
import { ReactNode } from "react";

interface ResourceProps {
  href: string;
  children?: ReactNode;
}

export default function Resource({ href, children }: ResourceProps) {
  return (
    <div className="w-full cursor-pointer overflow-hidden rounded-md border border-border transition-colors hover:bg-secondary/80">
      <Link href={href} className="flex size-full items-center gap-2 px-6 py-4">
        {children}
      </Link>
    </div>
  );
}
