"use client";

import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "./ui/dialog";
import NewStarForm from "./new-star-form";
import NewPlanetForm from "./new-planet-form";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "./ui/dropdown-menu";
import { Earth, Plus, Star as StarIcon } from "lucide-react";
import { Button } from "./ui/button";
import { useState } from "react";
import { Star } from "@/lib/schema";

export interface NewResourceBtnProps {
  galaxyId: string;
  stars: Star[];
}

type DialogType = "star" | "planet";

export default function NewResourceBtn({
  galaxyId,
  stars,
}: NewResourceBtnProps) {
  const [dialogType, setDialogType] = useState<DialogType>("star");

  function changeDialogType(type: DialogType) {
    return function () {
      setDialogType(type);
    };
  }

  return (
    <Dialog>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button className="flex items-center gap-2">
            <Plus className="size-4" />
            <span>New</span>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="p-2">
          <DialogTrigger asChild>
            <DropdownMenuItem
              className="flex cursor-pointer items-center gap-2"
              onClick={changeDialogType("star")}
            >
              <StarIcon className="size-4" />
              <span>New Star</span>
            </DropdownMenuItem>
          </DialogTrigger>
          <DialogTrigger asChild>
            <DropdownMenuItem
              className="flex cursor-pointer items-center gap-2"
              onClick={changeDialogType("planet")}
            >
              <Earth className="size-4" />
              <span>New Planet</span>
            </DropdownMenuItem>
          </DialogTrigger>
        </DropdownMenuContent>
      </DropdownMenu>
      {dialogType === "star" ? (
        <DialogContent>
          <DialogHeader>
            <DialogTitle>New Star</DialogTitle>
          </DialogHeader>
          <NewStarForm galaxyId={galaxyId} />
        </DialogContent>
      ) : (
        <DialogContent>
          <DialogHeader>
            <DialogTitle>New Planet</DialogTitle>
          </DialogHeader>
          <NewPlanetForm galaxyId={galaxyId} stars={stars} />
        </DialogContent>
      )}
    </Dialog>
  );
}
