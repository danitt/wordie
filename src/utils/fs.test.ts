import { assert } from "https://deno.land/std@0.70.0/testing/asserts.ts";
import { existsSync } from "https://deno.land/std/fs/mod.ts";
import { readJson, readTextFile, writeJson, writeTextFile } from './fs.ts';

Deno.test("Reads/writes text files", async () => {
  const testFilePath = "./output/testFile.csv";
  existsSync(testFilePath) && Deno.removeSync(testFilePath);
  const testData = "banana";
  await writeTextFile(testFilePath, testData);
  const readResult = await readTextFile(testFilePath);
  assert(testData === readResult);
});

Deno.test("Reads/writes json files", async () => {
  const testFilePath = "./output/testFile.json";
  existsSync(testFilePath) && Deno.removeSync(testFilePath);
  const testData = { banana: true };
  await writeJson(testFilePath, testData);
  const readResult = await readJson(testFilePath);
  assert(JSON.stringify(testData) === JSON.stringify(readResult));
});