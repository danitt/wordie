import type { Definition } from "./typings/definition.ts";
import { getFailedWords, getWordList } from "./utils/wordlist.ts";
import { readExistingDefinitions } from "./utils/definition.ts";
import { writeJson, writeTextFile } from "./utils/fs.ts";

/**
 * Reads given wordlist and generates a definition array at ./output/definitions.json
 * 
 * Note: if definitions already exists, will augment existing list
 */
export async function generateDefinitions(): Promise<void> {
  const definitions: Definition[] = await readExistingDefinitions();
  const failedWords: string[] = await getFailedWords();

  // Load word list, excluding any already defined
  const definedWords: string[] = definitions.map(d => d.word);
  const allWords = await getWordList();
  const words = allWords.filter(word => !definedWords.includes(word));

  // Validate word list
  if (!words || !words.length) {
    console.info('No new words to define');
    return;
  }

  // Iterate new words and fetch definitions
  for (const word of words) {
    // Get definition
    console.info('Defining word:', word);
    const wordDefinitions: Definition[] = await (await fetch(`https://api.dictionaryapi.dev/api/v2/entries/en/${word}`)).json();

    // Validate
    if (!wordDefinitions?.length) {
      // No definitions, add to failed words
      console.warn('Failed to retrieve definition for', word);
      failedWords.push(word);
      continue;
    }

    // Append to array
    definitions.push(wordDefinitions[0]);

    // Throttle queries
    await new Promise((resolve) => setTimeout(resolve, 500));
  }

  // Write definition file
  const definitionsPath = "./output/definitions.json";
  await writeJson(definitionsPath, { definitions });

  // Write failed words
  const failedWordsPath = "./output/failed.csv";
  await writeTextFile(failedWordsPath, failedWords.join('\n'));
}

await generateDefinitions();