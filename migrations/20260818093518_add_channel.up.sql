-- Add up migration script here
CREATE TABLE "channel"
(
    "id"       TEXT NOT NULL PRIMARY KEY,
    "guild_id" TEXT NOT NULL,
    FOREIGN KEY ("guild_id") REFERENCES "guild" ("id")
);
