import { ScriptsConfiguration } from "https://deno.land/x/velociraptor@1.2.0/mod.ts";

export default <ScriptsConfiguration>{
  scripts: {
    start: "deno run --allow-read --allow-write --allow-net --unstable src/define.ts",
    anki: "deno run --allow-read --allow-write --unstable src/anki.ts",
    test: "deno test --allow-read --allow-write --unstable"
  },
};