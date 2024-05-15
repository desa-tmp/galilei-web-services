"use client";

import { PlanetData, PlanetDataSchema, Star } from "@/lib/schema";
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
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "./ui/select";

interface PlanetFormProps {
  // eslint-disable-next-line no-unused-vars
  action: (data: PlanetData) => Promise<void>;
  stars: Star[];
  planet?: PlanetData;
}

const EMPTY_PLANET: PlanetData = {
  name: "",
  capacity: 5,
  path: "",
  star_id: "",
};

const DISCONNECTED_VALUE = "disconected" as const;

export default function PlanetForm({ action, stars, planet }: PlanetFormProps) {
  const form = useForm<PlanetData>({
    resolver: zodResolver(PlanetDataSchema),
    defaultValues: {
      ...EMPTY_PLANET,
      ...planet,
    },
  });

  async function onSubmit(data: PlanetData) {
    await action(data);
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Name</FormLabel>
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
          name="star_id"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Star</FormLabel>
              <Select
                onValueChange={field.onChange}
                value={
                  field.value === DISCONNECTED_VALUE ? undefined : field.value
                }
              >
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select a star to connect planet with" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem value={DISCONNECTED_VALUE}>No one</SelectItem>
                  <SelectGroup>
                    <SelectLabel>Stars</SelectLabel>
                    {stars.map(({ id, name }) => (
                      <SelectItem key={id} value={id}>
                        {name}
                      </SelectItem>
                    ))}
                  </SelectGroup>
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button
          type="submit"
          className="w-full"
          loading={form.formState.isSubmitting}
        >
          Submit
        </Button>
      </form>
    </Form>
  );
}
