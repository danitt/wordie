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
  const definitionsPath = "./output/definitions.json";
  const failedWords: string[] = await getFailedWords();
  const failedWordsPath = "./output/failed.csv";

  // Load word list, excluding any already defined
  const definedWords: string[] = definitions.map(d => String(d.word).toLowerCase());
  const allWords = await getWordList();
  const words = allWords
    .map(word => String(word).toLowerCase())
    .filter(word => !definedWords.includes(word));

  // Validate word list
  if (!words || !words.length) {
    console.info('No new words to define');
    return;
  }

  // Iterate new words and fetch definitions
  console.info(`Found ${words.length} new words to define`);
  for (const word of words) {
    // Get definition
    console.info('Defining word:', word);
    const wordDefinitions: Definition[] = await (await fetch(`https://api.dictionaryapi.dev/api/v2/entries/en/${word}`)).json();

    // Validate
    if (!wordDefinitions?.length) {
      // No definitions, add to failed words
      console.warn('Failed to retrieve definition for', word);
      failedWords.push(word);
      await writeTextFile(failedWordsPath, failedWords.join("\n"));
      continue;
    }
    
    // Update definitions file
    definitions.push(wordDefinitions[0]);
    await writeJson(definitionsPath, { definitions });

    // Throttle queries
    await new Promise((resolve) => setTimeout(resolve, 500));
  }

}

await generateDefinitions();