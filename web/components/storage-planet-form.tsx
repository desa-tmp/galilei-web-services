"use client";

import { PlanetDataSchema } from "@/lib/schema";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "./ui/form";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { z } from "zod";
import { updatePlanet } from "@/lib/actions";

const StoragePlanetDataSchema = PlanetDataSchema.pick({
  path: true,
  capacity: true,
});

type StoragePlanetData = z.infer<typeof StoragePlanetDataSchema>;

interface PlanetFormProps {
  galaxyId: string;
  planetId: string;
  storageData: StoragePlanetData;
}

export default function StoragePlanetForm({
  galaxyId,
  planetId,
  storageData,
}: PlanetFormProps) {
  const form = useForm<StoragePlanetData>({
    resolver: zodResolver(StoragePlanetDataSchema),
    defaultValues: { ...storageData },
  });

  async function onSubmit(data: StoragePlanetData) {
    await updatePlanet(galaxyId, planetId, data);
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="path"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Path</FormLabel>
              <FormControl>
                <Input type="text" autoComplete="off" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="capacity"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Capacity</FormLabel>
              <FormControl>
                <Input type="number" autoComplete="off" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button
          type="submit"
          className="w-full"
          loading={form.formState.isSubmitting}
        >
          Update
        </Button>
      </form>
    </Form>
  );
}
