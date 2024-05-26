"use client";

import { useTransition } from "react";
import { Button } from "./ui/button";
import { ClipboardCopy } from "lucide-react";

interface CopyBtnProps {
  text: string;
  className?: string;
  disabled?: boolean;
}

export default function CopyBtn({
  text,
  className,
  disabled = false,
}: CopyBtnProps) {
  const [isPending, startTransition] = useTransition();

  function handleClick() {
    startTransition(async () => {
      await navigator.clipboard.writeText(text);
    });
  }

  return (
    <Button
      variant="ghost"
      size="icon"
      className={className}
      loading={isPending}
      onClick={handleClick}
      disabled={disabled}
    >
      <ClipboardCopy />
    </Button>
  );
}
