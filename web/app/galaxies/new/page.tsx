"use client";

import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { NewGalaxySchema, NewGalaxy } from "@/lib/schema";
import { newGalaxy } from "@/lib/actions";

export default function NewGalaxyPage() {
  const form = useForm<NewGalaxy>({
    resolver: zodResolver(NewGalaxySchema),
    defaultValues: {
      name: "",
    },
  });

  async function onSubmit(data: NewGalaxy) {
    await newGalaxy(data);
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <Form.Field
          control={form.control}
          name="name"
          render={({ field }) => (
            <Form.Item>
              <Form.Label>Galaxy name</Form.Label>
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
