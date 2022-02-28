import "reflect-metadata";
import { buildSchema } from "type-graphql";
import { createYamlResolver } from "./yaml-data/resolver";
import { Person } from "../schemata/person";

export const getSchema = async () =>
  await buildSchema({
    resolvers: [createYamlResolver("people", Person)],
    emitSchemaFile: {
      path: __dirname + "/../generated/schema.graphql",
      commentDescriptions: true,
      sortedSchema: false,
    },
  });
