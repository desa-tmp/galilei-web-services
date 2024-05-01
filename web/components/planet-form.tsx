"use client";

import { PlanetData, PlanetDataSchema } from "@/lib/schema";
import { Form } from "./ui/form";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "./ui/input";
import { Button } from "./ui/button";

interface PlanetFormProps {
  // eslint-disable-next-line no-unused-vars
  action: (data: PlanetData) => Promise<void>;
  planet?: PlanetData;
}

const EMPTY_PLANET: PlanetData = {
  name: "",
  capacity: 5,
  star_id: "",
};

export default function PlanetForm({ action, planet }: PlanetFormProps) {
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
        <Form.Field
          control={form.control}
          name="name"
          render={({ field }) => (
            <Form.Item>
              <Form.Label>Name</Form.Label>
              <Form.Control>
                <Input type="text" autoComplete="off" {...field} />
              </Form.Control>
              <Form.Message />
            </Form.Item>
          )}
        />
        <Form.Field
          control={form.control}
          name="capacity"
          render={({ field }) => (
            <Form.Item>
              <Form.Label>Capacity</Form.Label>
              <Form.Control>
                <Input type="number" autoComplete="off" {...field} />
              </Form.Control>
              <Form.Message />
            </Form.Item>
          )}
        />
        <Form.Field
          control={form.control}
          name="star_id"
          render={({ field }) => (
            <Form.Item>
              <Form.Label>Star</Form.Label>
              <Form.Control>
                <Input type="text" autoComplete="off" {...field} />
              </Form.Control>
              <Form.Message />
            </Form.Item>
          )}
        />
        <Button type="submit" className="w-full">
          Submit
        </Button>
      </form>
    </Form>
  );
}
