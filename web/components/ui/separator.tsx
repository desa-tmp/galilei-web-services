"use client";

import * as React from "react";
import * as SeparatorPrimitive from "@radix-ui/react-separator";

import { cn } from "@/lib/utils";

export interface SeparatorProps
  extends React.ComponentPropsWithoutRef<typeof SeparatorPrimitive.Root> {
  size?: number;
  children?: React.ReactNode;
}

const Separator = React.forwardRef<
  React.ElementRef<typeof SeparatorPrimitive.Root>,
  SeparatorProps
>(
  (
    {
      className,
      orientation = "horizontal",
      decorative = true,
      size = 1,
      style,
      children,
      ...props
    },
    ref
  ) => (
    <SeparatorPrimitive.Root
      ref={ref}
      decorative={decorative}
      orientation={orientation}
      className={cn(
        "relative shrink-0 bg-border",
        orientation === "horizontal" ? "w-full" : "h-full",
        className
      )}
      style={{
        ...style,
        [orientation === "horizontal" ? "height" : "width"]: size,
      }}
      {...props}
    >
      {children ? (
        <div className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
          {children}
        </div>
      ) : null}
    </SeparatorPrimitive.Root>
  )
);
Separator.displayName = SeparatorPrimitive.Root.displayName;

export { Separator };
