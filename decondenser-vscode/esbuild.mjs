#!/usr/bin/env node
//@ts-check

import * as esbuild from "esbuild";

const prod = process.env.MODE === "prod";
const platform = process.argv.includes("--browser") ? "browser" : "node";

await esbuild.build({
    target: "es2020",
    format: "cjs",
    outfile: `dist/extension.js`,
    entryPoints: ["src/extension.ts"],
    external: ["vscode"],
    bundle: true,
    sourcesContent: false,
    minify: prod,
    sourcemap: !prod,
    platform,
});
