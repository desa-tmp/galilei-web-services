"use client";

import { PlanetData, PlanetDataSchema, Star } from "@/lib/schema";
import { Form } from "./ui/form";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { Select } from "./ui/select";

interface PlanetFormProps {
  // eslint-disable-next-line no-unused-vars
  action: (data: PlanetData) => Promise<void>;
  stars: Star[];
  planet?: PlanetData;
}

const EMPTY_PLANET: PlanetData = {
  name: "",
  capacity: 5,
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
              <Select
                onValueChange={field.onChange}
                value={
                  field.value === DISCONNECTED_VALUE ? undefined : field.value
                }
              >
                <Form.Control>
                  <Select.Trigger>
                    <Select.Value placeholder="Select a star to connect planet with" />
                  </Select.Trigger>
                </Form.Control>
                <Select.Content>
                  <Select.Item value={DISCONNECTED_VALUE}>No one</Select.Item>
                  <Select.Group>
                    <Select.Label>Stars</Select.Label>
                    {stars.map(({ id, name }) => (
                      <Select.Item key={id} value={id}>
                        {name}
                      </Select.Item>
                    ))}
                  </Select.Group>
                </Select.Content>
              </Select>
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
