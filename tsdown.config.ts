import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["externals/index.ts"],
  format: ["cjs", "esm"],
  dts: true,
  shims: true,
  clean: true,
  outDir: "dist",
});
