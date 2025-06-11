-- Your SQL goes here
CREATE TABLE "users"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"username" TEXT NOT NULL,
	"password_hash" TEXT NOT NULL,
	"jwt_intra_epitech" TEXT,
	"jwt_expires_at" TIMESTAMP
);

