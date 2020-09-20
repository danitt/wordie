import { assert } from "https://deno.land/std@0.70.0/testing/asserts.ts";
import { existsSync } from "https://deno.land/std/fs/mod.ts";
import { writeJson } from "./fs.ts";
import { readExistingDefinitions } from "./definition.ts";
import type { Definition } from "../typings/definition.ts";

Deno.test("Reads existing definitions", async () => {
  const testExistingDefinitionsPath = "./output/test/definitions.json";
  const testDefinitions: Definition[] = [{
    word: 'test',
    phonetics: [],
    meanings: [],
  }];

  // If not failed word list exists, should return empty string []
  existsSync(testExistingDefinitionsPath) && Deno.removeSync(testExistingDefinitionsPath);
  const definitionsEmpty = await readExistingDefinitions(testExistingDefinitionsPath);
  assert(Array.isArray(definitionsEmpty) && definitionsEmpty.length === 0);

  // Populate failed words list, ensure returns correctly
  await writeJson(testExistingDefinitionsPath, { definitions: testDefinitions });

  // Check that test content is parsed to string array
  const existingDefinitions = await readExistingDefinitions(testExistingDefinitionsPath);
  assert(Array.isArray(existingDefinitions) && existingDefinitions[0].word === testDefinitions[0].word);
});