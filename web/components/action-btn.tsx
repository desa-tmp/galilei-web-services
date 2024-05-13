"use client";

import { cn } from "@/lib/utils";
import { ComponentProps, ElementRef, forwardRef, useTransition } from "react";
import { Button } from "./ui/button";

interface ActionBtnProps extends ComponentProps<typeof Button> {
  // eslint-disable-next-line no-unused-vars
  action: (...args: unknown[]) => Promise<unknown>;
}

const ActionBtn = forwardRef<ElementRef<"button">, ActionBtnProps>(
  ({ action, onClick, className, children, ...props }, ref) => {
    const [isPending, startTransition] = useTransition();

    return (
      <Button
        ref={ref}
        className={cn(className, "cursor-pointer")}
        onClick={(e) => {
          onClick?.(e);
          startTransition(async () => {
            await action();
          });
        }}
        loading={isPending}
        {...props}
      >
        {children}
      </Button>
    );
  }
);
ActionBtn.displayName = "ActionBtn";

export default ActionBtn;
