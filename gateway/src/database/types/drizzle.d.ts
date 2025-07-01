import { MySqlDatabase } from "drizzle-orm/mysql-core";
import * as schema from "../schema";
import { PgDatabase } from "drizzle-orm/pg-core";
import { ExtractTablesWithRelations } from "drizzle-orm";

export type DrizzleDB = PgDatabase<any, typeof schema, ExtractTablesWithRelations<typeof schema>>;
