import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["externals/**/*"],
  format: ["cjs", "esm"],
  dts: true,
  shims: true,
  clean: true,
  unbundle: true,
  exports: true,
  skipNodeModulesBundle: true,
});
