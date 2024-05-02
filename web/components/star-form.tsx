"use client";

import { StarData, StarDataSchema } from "@/lib/schema";
import { Form } from "./ui/form";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "./ui/input";
import { Button } from "./ui/button";

interface StarFormProps {
  // eslint-disable-next-line no-unused-vars
  action: (data: StarData) => Promise<void>;
  star?: StarData;
}

const EMPTY_STAR: StarData = {
  name: "",
  nebula: "",
};

export default function StarForm({ action, star }: StarFormProps) {
  const form = useForm<StarData>({
    resolver: zodResolver(StarDataSchema),
    defaultValues: { ...EMPTY_STAR, ...star },
  });

  async function onSubmit(data: StarData) {
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
          name="nebula"
          render={({ field }) => (
            <Form.Item>
              <Form.Label>Nebula</Form.Label>
              <Form.Control>
                <Input type="text" autoComplete="off" {...field} />
              </Form.Control>
              <Form.Message />
            </Form.Item>
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
