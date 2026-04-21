const esbuild = require("esbuild");

esbuild
  .build({
    entryPoints: ["main.ts"],
    bundle: true,
    outfile: "dist/main.js",
    platform: "neutral",
    target: "es2017",
    external: ["k6", "k6/*"],
  })
  .catch(() => process.exit(1));
