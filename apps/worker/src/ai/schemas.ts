import { z } from "zod";

export const scheduleExtractionSchema = z.object({
  entries: z.array(z.object({ title: z.string().min(1), format: z.enum(["SHOW", "PRESENTATION", "FILM", "OTHER"]), startTime: z.string(), endTime: z.string().optional(), timezone: z.string().min(1), officialUrl: z.string().url().optional() }))
});
export const collectionExtractionSchema = z.object({
  name: z.string().min(1), maisonName: z.string().min(1), seasonCode: z.string().regex(/^(SS|FW)\d{2,4}$/), seasonYear: z.number().int(), seasonLabel: z.string().min(1), presentedOn: z.string().optional(), sourceNotes: z.string().optional()
});

export function validateAIOutput<T>(schema: z.ZodType<T>, value: unknown): T { return schema.parse(value); }
