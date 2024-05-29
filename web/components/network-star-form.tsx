"use client";

import { StarDataSchema } from "@/lib/schema";
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
import { updateStar } from "@/lib/actions";
import CopyBtn from "./copy-btn";
import { Label } from "./ui/label";
import { ScrollArea, ScrollBar } from "./ui/scroll-area";

const NetworkStarDataSchema = StarDataSchema.pick({
  public_domain: true,
  port: true,
});

type NetworkStarData = z.infer<typeof NetworkStarDataSchema>;

interface NetworkStarFormProps {
  galaxyId: string;
  starId: string;
  networkData: NetworkStarData;
}

export default function NetworkStarForm({
  galaxyId,
  starId,
  networkData,
}: NetworkStarFormProps) {
  const form = useForm<NetworkStarData>({
    resolver: zodResolver(NetworkStarDataSchema),
    defaultValues: { ...networkData },
  });

  async function onSubmit(data: NetworkStarData) {
    await updateStar(galaxyId, starId, data);
  }

  const privateDomain = `star-${starId}.galaxy-${galaxyId}.svc.cluster.local`;

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="public_domain"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Public Domain</FormLabel>
              <div className="flex items-center gap-4">
                <FormControl>
                  <Input
                    type="text"
                    autoComplete="off"
                    className="flex-1"
                    {...field}
                  />
                </FormControl>
                <CopyBtn
                  text={`${field.value}.localhost`}
                  disabled={field.value.length === 0}
                />
              </div>
              <FormMessage />
            </FormItem>
          )}
        />
        <div className="space-y-2">
          <Label>Private Domain</Label>
          <div className="flex items-center gap-4">
            <ScrollArea className="flex-1 px-2 py-1">
              <span className="text-nowrap">{privateDomain}</span>
              <ScrollBar orientation="horizontal" />
            </ScrollArea>
            <CopyBtn text={privateDomain} />
          </div>
        </div>
        <FormField
          control={form.control}
          name="port"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Internal port</FormLabel>
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
