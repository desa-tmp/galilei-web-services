import { Layout } from "@/lib/types";

export default function GalaxyLayout({ children, details }: Layout<"details">) {
  console.log("here");
  return (
    <div className="flex">
      <div>{children}</div>
      <div>{details}</div>
    </div>
  );
}
