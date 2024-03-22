import * as esbuild from "esbuild";
import { sassPlugin } from "esbuild-sass-plugin";
import postcss from "postcss";
import autoprefixer from "autoprefixer";
import postcssPresetEnv from "postcss-preset-env";

await esbuild.build({
    entryPoints: [
        "src/index.ts",
        "src/styles/*.scss",
    ],
    outdir: "../build",
    bundle: true,
    legalComments: "inline",
    logLevel: "debug",
    minify: true,
    format: "esm",
    splitting: true,
    keepNames: true,
    outExtension: {
        ".js": ".min.js",
        ".css": ".min.css",
    },
    sourcemap: true,
    pure: ["console", "throw"],
    platform: "browser",
    plugins: [
        sassPlugin({
            async transform(source, resolveDir, filePath) {
                const { css } = await postcss([
                    autoprefixer,
                    postcssPresetEnv({ stage: 0 })
                ]).process(source, { from: filePath })

                return css;
            },
        })
    ],
    external: ["*.ttf"]
});
