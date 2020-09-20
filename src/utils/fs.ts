import { ensureFileSync } from "https://deno.land/std/fs/mod.ts";

/**
 * Reads file content at given path, returning undefined if not found or error
 */
export async function readTextFile(filePath: string): Promise<string> {
  try {
    return Deno.readTextFileSync(filePath);
  } catch (e) {
    console.error("Unable to read file");
    throw Error(e);
  }
}

/**
 * Writes data to the specified file path, overwriting if exists
 */
export async function writeTextFile(
  relativePath: string,
  data: string,
): Promise<void> {
  try {
    ensureFileSync(relativePath);
    Deno.writeTextFileSync(relativePath, data);
  } catch (e) {
    console.error("Error writing file to path");
    throw Error(e);
  }
}

/**
 * Writes data to the specified file path, overwriting if exists
 */
export async function writeJson(relativePath: string, data: unknown): Promise<void> {
  try {
    const dataStr = JSON.stringify(data);
    await writeTextFile(relativePath, dataStr);
  } catch (e) {
    console.error("Error writing file to path");
    throw Error(e);
  }
}

/**
 * Reads JSON content at given path, returning undefined if not found or error
 */
export async function readJson<Contents = unknown>(filePath: string): Promise<Contents | void> {
  try {
    const fileData = await readTextFile(filePath);
    return JSON.parse(fileData);
  } catch (e) {
    console.error("Unable to read JSON");
    throw Error(e);
  }
}