import { existsSync } from "https://deno.land/std@0.105.0/fs/mod.ts";

import type { Definition, DefinitionFile } from "../typings/definition.ts";
import { readJson } from "./fs.ts";

/**
 * Reads existing definition entries, or returns empty array
 */
export async function readExistingDefinitions(definitionsFilePath = "./output/definitions.json"): Promise<Definition[]> {
  try {
    if (!existsSync(definitionsFilePath)) {
      // No existing definitions, return empty array
      return [];
    }
    // Load existing definitions
      const definitionFile = await readJson<DefinitionFile>(definitionsFilePath);
      return definitionFile.definitions;
  } catch (e) {
    console.error('Error loading existing definitions');
    throw Error(e);
  }
}