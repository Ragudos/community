import * as esbuild from "esbuild";

const h = await esbuild.build({
    entryPoints: [
        "src/index.ts",
        "src/styles/*.css"
    ],
    "bundle": true,
    "outdir": "../build",
    "logLevel": "debug",
    "format": "esm",
    "splitting": true,
    "keepNames": true,
    "outExtension": {
        ".js": ".min.mjs",
        ".css": ".min.css"
    },
    "pure": ["console.error", "console.log", "throw"],
    "minifyWhitespace": true,
    "minifyIdentifiers": true
});

console.log(h);
