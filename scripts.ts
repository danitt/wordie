import { ScriptsConfiguration } from "https://deno.land/x/velociraptor@v1.0.0-beta.13/mod.ts";

export default <ScriptsConfiguration>{
  scripts: {
    start: "deno run --allow-read --allow-write --allow-net --unstable src/define.ts",
    test: "deno test --allow-read --allow-write --unstable"
  },
};