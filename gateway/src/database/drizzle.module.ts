import { Logger, Module } from "@nestjs/common";
import env from "src/common";
import * as schema from "./schema";
import * as path from "path";
import { Pool } from "pg";
import { drizzle } from "drizzle-orm/node-postgres";
import { migrate } from "drizzle-orm/node-postgres/migrator";
import { PgDatabase } from "drizzle-orm/pg-core";
import { ExtractTablesWithRelations } from "drizzle-orm";

export const DRIZZLE = Symbol("Drizzle Connection");

@Module({
  providers: [
    {
      provide: DRIZZLE,
      useFactory: async (): Promise<PgDatabase<any, typeof schema, ExtractTablesWithRelations<typeof schema>>> => {
        const logger: Logger = new Logger("DatabaseMigrator");

        const pool = new Pool({
          connectionString: env.DATABASE_URL,
        });

        const db = drizzle({ client: pool, schema });

        try {
          await migrate(db, {
            migrationsFolder: path.join(__dirname, "..", "..", "migrations")
          });
          logger.log("Database migrated successfully.");
        } catch (error) {
          console.error("Migration failed:", error);
          logger.log("Database is already up to date or migration failed.");
        }

        return db;
      },
    },
  ],
  exports: [DRIZZLE],
})
export class DrizzleModule { }
