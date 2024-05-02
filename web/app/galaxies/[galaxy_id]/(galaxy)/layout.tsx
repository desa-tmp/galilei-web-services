"use client";

import { Layout } from "@/lib/types";
import { Resizable } from "@/components/ui/resizable";
import { useSelectedLayoutSegment } from "next/navigation";

export default function GalaxyLayout({ children, details }: Layout<"details">) {
  const segment = useSelectedLayoutSegment("details");
  const galaxyPanelDefaultSize = segment === "children" ? 40 : 100;

  return (
    <Resizable direction="horizontal">
      <Resizable.Panel
        id="galaxy"
        minSize={30}
        defaultSize={galaxyPanelDefaultSize}
        order={1}
      >
        {children}
      </Resizable.Panel>
      {segment === "children" && (
        <>
          <Resizable.Handle withHandle />
          <Resizable.Panel id="details" minSize={30} defaultSize={60} order={2}>
            {details}
          </Resizable.Panel>
        </>
      )}
    </Resizable>
  );
}
