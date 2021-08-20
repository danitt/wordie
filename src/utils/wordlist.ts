import { parse } from "https://deno.land/std@0.105.0/encoding/csv.ts";
import { existsSync } from "https://deno.land/std@0.105.0/fs/mod.ts";
import { readTextFile } from "./fs.ts";

/**
 * Reads word list (words.csv), and parses to string array
 */
export async function getWordList(wordlistPath = './words.csv'): Promise<string[]> {
  try {
    const wordFileStr = await readTextFile(wordlistPath);
    const wordlistParsed = await parse(wordFileStr, {
      skipFirstRow: false,
    }) as string[][];
    const words: string[] = wordlistParsed
      // Extract first value from every line
      .map((ln) => ln?.[0] ?? '')
      // Remove any empty values
      .filter(w => !!w);
    return words;
  } catch (e) {
    console.error('Could not read word list - please check valid words.csv file exists at project route');
    throw (e);
  }
}

/**
 * Get failed word list if exists (./output/failed.csv), else return empty string[]
 */
export async function getFailedWords(failedWordsPath = "./output/failed.csv"): Promise<string[]> {
  try {
    if (!existsSync(failedWordsPath)) {
      // No existing failed words exist, just return empty string array
      return [];
    }
    const wordFileStr = await readTextFile(failedWordsPath);
    const wordlistParsed = await parse(wordFileStr, {
      skipFirstRow: false,
    }) as string[][];
    const words: string[] = wordlistParsed
      // Extract first value from every line
      .map((ln) => ln?.[0] ?? "")
      // Remove any empty values
      .filter((w) => !!w);
    return words;
  } catch (e) {
    console.error("Could not read failed word list");
    throw (e);
  }
}