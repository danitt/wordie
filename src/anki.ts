import type { Definition } from "./typings/definition.ts";
import { readExistingDefinitions } from "./utils/definition.ts";
import { writeTextFile } from "./utils/fs.ts";

/**
 * Reads definitions list, and converts to ./output/anki.csv for Anki flash cards import
 */
export async function generateAnkiCards(): Promise<void> {
  const definitions: Definition[] = await readExistingDefinitions();
  const cards: string[] = [];

  for (const definition of definitions) {
    // Front of card (word)
    const front = definition.word;
    // Prepare list of meanings
    const meaningsHtml = definition.meanings.map((meaning, meaningIdx) => {
      return meaning.definitions.map((wordDefinition, idx) => {
        const partOfSpeechHtml = meaning.partOfSpeech && `${meaningIdx + 1}. <em>(${meaning.partOfSpeech})</em>`;
        const definitionHtml = wordDefinition.definition && `<strong>Definition:</strong> ${wordDefinition.definition}`;
        const exampleHtml = wordDefinition.example && `<strong>Example:</strong> ${wordDefinition.example}`;
        return [
          partOfSpeechHtml,
          definitionHtml,
          exampleHtml,
          '<br />',
        ].join('<br />');
      }).join('<br />');
    }).join('<br />');
    // Concatentate back of card (i.e. the definition/s)
    const back = `<div style="text-align: left;">${meaningsHtml}</div>`;
    // Prepare card
    const card = String(`${front}|${back}`)
      // Strip line breaks
      .replace(/(\r\n|\n|\r)/gm,"");
    cards.push(card);
  }

  // Write to file
  await writeTextFile("./output/anki.txt", cards.join("\n"));
}

await generateAnkiCards();