-- This file should undo anything in `up.sql`

ALTER TABLE "guild" ADD COLUMN "honeypot_channel_id" TEXT;

DROP TABLE IF EXISTS "honeypot";
