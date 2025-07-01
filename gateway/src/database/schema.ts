import { pgTable, pgView, text, boolean } from "drizzle-orm/pg-core";

export const users = pgTable("users", {
  id: text().$default(() => crypto.randomUUID()).primaryKey(),
  username: text().unique().notNull(),
  name: text().notNull().$default(() => "User_" + crypto.randomUUID().substring(0, 5)),
  email: text().unique().notNull(),
  isVerified: boolean("is_verified").$default(() => false).notNull(),
  password: text().notNull(),
  role: text({ enum: ["ADMIN", "USER"] }).$default(() => "USER").notNull(),
  refreshToken: text("refresh_token"),
  createdAt: text("created_at").$default(() => new Date().toISOString()).notNull(),
  updatedAt: text("updated_at").$default(() => new Date().toISOString()).notNull(),
});

export const usersView = pgView("user_views").as((qb) => qb.select({
  id: users.id,
  username: users.username,
  name: users.name,
  role: users.role,
}).from(users));

