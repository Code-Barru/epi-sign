-- Your SQL goes here

CREATE TABLE "cookies"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"date" DATE NOT NULL,
	"cookie_data" JSONB NOT NULL
);

