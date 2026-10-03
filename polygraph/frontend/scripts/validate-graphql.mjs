// SPDX-License-Identifier: MPL-2.0

import { readFile } from "node:fs/promises";
import { buildSchema, parse, validate } from "graphql";

const [schemaSource, mutationSource] = await Promise.all([
  readFile(new URL("../src/graphql/schema.graphql", import.meta.url), "utf8"),
  readFile(new URL("../src/graphql/Mutations.res", import.meta.url), "utf8"),
]);

const documentMatch = mutationSource.match(/Client\.parse\("([^"]+)"\)/);
if (!documentMatch) {
  throw new Error("VerifyClaimMutation must contain a Client.parse GraphQL document");
}

const schema = buildSchema(schemaSource);
const document = parse(documentMatch[1]);
const errors = validate(schema, document);

if (errors.length > 0) {
  for (const error of errors) console.error(error.message);
  process.exit(1);
}

if (document.definitions.length !== 1 || document.definitions[0].operation !== "mutation") {
  throw new Error("VerifyClaimMutation must contain exactly one mutation operation");
}

console.log("GraphQL verify mutation matches the frontend/backend contract.");
