CREATE TYPE package AS ENUM ('whole', 'partial');

CREATE TABLE "pantry" (
  "upc" bigint PRIMARY KEY,
  "amount" int,
  "unit" varchar,
  "package_type" package,
  "brand" varchar
);

CREATE TABLE "tags" (
  "id" SERIAL PRIMARY KEY,
  "food" int,
  "upc" bigint
);

CREATE TABLE "food" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar,
  "desc" text
);

CREATE TABLE "recipes" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar,
  "directions" text,
  "author" varchar,
  "source" varchar
);

CREATE TABLE "ingredients" (
  "id" SERIAL PRIMARY KEY,
  "recipe" int,
  "tag" int,
  "amount" int,
  "unit" varchar
);

ALTER TABLE "tags" ADD FOREIGN KEY ("upc") REFERENCES "pantry" ("upc");

ALTER TABLE "ingredients" ADD FOREIGN KEY ("recipe") REFERENCES "recipes" ("id");

ALTER TABLE "ingredients" ADD FOREIGN KEY ("tag") REFERENCES "food" ("id");

ALTER TABLE "tags" ADD FOREIGN KEY ("food") REFERENCES "food" ("id");
