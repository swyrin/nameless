-- Add up migration script here
ALTER TABLE "guild"
    DROP COLUMN "honeypot_channel_id";

CREATE TABLE "honeypot"
(
    "guild_id"   TEXT    NOT NULL,
    "channel_id" TEXT    NOT NULL,
    "enabled"    BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE ("guild_id"),
    PRIMARY KEY ("guild_id", "channel_id"),
    FOREIGN KEY ("guild_id") REFERENCES "guild" ("id"),
    FOREIGN KEY ("channel_id") REFERENCES "channel" ("id")
);

