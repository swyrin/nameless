-- Add down migration script here
ALTER TABLE "guild"
    ADD COLUMN "honeypot_channel_id" TEXT;

DROP TABLE IF EXISTS "honeypot";
