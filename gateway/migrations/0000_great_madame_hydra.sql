CREATE TABLE "users" (
	"id" text PRIMARY KEY NOT NULL,
	"username" text NOT NULL,
	"name" text NOT NULL,
	"email" text NOT NULL,
	"is_verified" boolean NOT NULL,
	"password" text NOT NULL,
	"role" text NOT NULL,
	"refresh_token" text,
	"created_at" text NOT NULL,
	"updated_at" text NOT NULL,
	CONSTRAINT "users_username_unique" UNIQUE("username"),
	CONSTRAINT "users_email_unique" UNIQUE("email")
);
--> statement-breakpoint
CREATE VIEW "public"."user_views" AS (select "id", "username", "name", "role" from "users");