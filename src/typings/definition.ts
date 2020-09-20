export interface DefinitionFile {
  definitions: Definition[];
};

export interface Definition {
  word: string;
  phonetics: {
    // Phonetic pronounciation
    text: string;
    // Path to audio file
    audio: string;
  }[];
  meanings: {
    partOfSpeech: string;
    definitions: {
      definition: string;
      example: string;
      synonyms: string[];
    }[];
  }[];
}