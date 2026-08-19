-- Add up migration script here
CREATE TABLE "guild"
(
    "id"                  TEXT NOT NULL PRIMARY KEY,
    "honeypot_channel_id" TEXT
);
