import "dotenv/config";
import { z } from "zod";

const schema = z.object({
  GATEWAY_PORT: z.coerce.number().default(8000),
  NODE_ENV: z.enum(["production", "development"]).default("development"),

  DATABASE_URL: z.string({ required_error: "DATABASE_URL is required" }),

  JWT_ACCESS_SECRET: z.string({ required_error: "JWT_ACCESS_SECRET is required" }),
  JWT_REFRESH_SECRET: z.string({ required_error: "JWT_REFRESH_SECRET is required" }),
});

export type Env = z.infer<typeof schema>;

let parsed: Env;
try {
  parsed = schema.parse(process.env);
} catch (error) {
  if (error instanceof z.ZodError) {
    console.error(
      "❌ Invalid environment variables:",
      JSON.stringify(error.errors, null, 2),
    );
  } else {
    console.error("❌ Error parsing environment variables:", error);
  }
  process.exit(1);
}

const env = parsed;

export default env;
