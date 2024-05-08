"use client";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { GalaxyDataSchema, GalaxyData } from "@/lib/schema";

interface GalaxyFormProps {
  // eslint-disable-next-line no-unused-vars
  action: (data: GalaxyData) => Promise<void>;
  galaxy?: GalaxyData;
}

const EMPTY_GALAXY: GalaxyData = {
  name: "",
};

export default function GalaxyForm({ action, galaxy }: GalaxyFormProps) {
  const form = useForm<GalaxyData>({
    resolver: zodResolver(GalaxyDataSchema),
    defaultValues: {
      ...EMPTY_GALAXY,
      ...galaxy,
    },
  });

  async function onSubmit(data: GalaxyData) {
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
              <FormLabel>Galaxy name</FormLabel>
              <FormControl>
                <Input type="text" autoComplete="off" {...field} />
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
          Submit
        </Button>
      </form>
    </Form>
  );
}
