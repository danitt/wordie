import { assert } from "https://deno.land/std@0.70.0/testing/asserts.ts";
import { existsSync } from "https://deno.land/std/fs/mod.ts";
import { writeTextFile } from "./fs.ts";
import { getWordList } from "./wordlist.ts";

Deno.test("Reads wordlist", async () => {
  // Create fake words.csv to test with
  const testWordlistPath = './output/test/testWordlist.csv';
  existsSync(testWordlistPath) && Deno.removeSync(testWordlistPath);
  const testWords = ['test', 'wordlist', 'banana'];
  const testWordlist = testWords.join('\n');
  await writeTextFile(testWordlistPath, testWordlist);

  // Check that test content is parsed to string array
  const words = await getWordList(testWordlistPath);

  assert(Array.isArray(words));
  assert(testWords.every(w => words.includes(w)));
});