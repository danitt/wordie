import { assert } from "https://deno.land/std@0.70.0/testing/asserts.ts";
import { existsSync } from "https://deno.land/std/fs/mod.ts";
import { writeTextFile } from "./fs.ts";
import { getFailedWords, getWordList } from "./wordlist.ts";

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

Deno.test("Reads failed words", async () => {
  const testFailedWordsPath = "./output/test/failed.csv";
  const testFailedWords = ["cheesecake", "philology", "ragamuffin"];

  // If not failed word list exists, should return empty string []
  existsSync(testFailedWordsPath) && Deno.removeSync(testFailedWordsPath);
  const failedWordsEmpty = await getFailedWords(testFailedWordsPath);
  assert(Array.isArray(failedWordsEmpty) && failedWordsEmpty.length === 0);

  // Populate failed words list, ensure returns correctly
  const testFailedWordlist = testFailedWords.join("\n");
  await writeTextFile(testFailedWordsPath, testFailedWordlist);

  // Check that test content is parsed to string array
  const failedWords = await getFailedWords(testFailedWordsPath);
  assert(Array.isArray(failedWords));
  assert(testFailedWords.every((w) => failedWords.includes(w)));
});