import * as esbuild from "esbuild";

const h = await esbuild.build({
    entryPoints: [
        "src/index.ts",
        "src/crop.ts",
        "src/styles/*.css",
        "src/workers/*.ts",
    ],
    bundle: true,
    outdir: "../build",
    logLevel: "debug",
    format: "esm",
    splitting: true,
    keepNames: true,
    outExtension: {
        ".js": ".min.mjs",
        ".css": ".min.css",
    },
    pure: ["console.error", "console.log", "throw"],
    minifyWhitespace: true,
    minifyIdentifiers: true,
});

console.log(h);
