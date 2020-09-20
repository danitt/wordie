import { parse } from "https://deno.land/std/encoding/csv.ts";
import { readTextFile } from "./fs.ts";

/**
 * Reads word list (words.csv), and parses to string array
 */
export async function getWordList(wordlistPath = './words.csv'): Promise<string[]> {
  try {
    const wordFileStr = await readTextFile(wordlistPath);
    const wordlistParsed = await parse(wordFileStr, {
      header: false,
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