"use client";

import { HTMLAttributes, MouseEvent } from "react";

interface ActionClickProps extends HTMLAttributes<HTMLButtonElement> {
  // eslint-disable-next-line no-unused-vars
  action: (...args: unknown[]) => Promise<unknown>;
}

export default function ActionClick({
  action,
  onClick,
  children,
  ...props
}: ActionClickProps) {
  async function onClickWithAction(e: MouseEvent<HTMLButtonElement>) {
    onClick?.(e);
    await action();
  }

  return (
    <button onClick={onClickWithAction} {...props}>
      {children}
    </button>
  );
}
