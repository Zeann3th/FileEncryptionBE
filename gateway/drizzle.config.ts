import "dotenv/config";
import { defineConfig } from "drizzle-kit";
import env from "src/common";

export default defineConfig({
  schema: "./src/database/schema.ts",
  out: "./migrations",
  dialect: "postgresql",
  dbCredentials: {
    url: env.DATABASE_URL
  }
});
