import { spawn } from "node:child_process";

import { ChildProcess } from "@figliolia/child-process";

import type { CriticalPathAnalysis } from "./types";

/**
 * analyzeCriticalPath
 *
 * Returns a critical path analysis containing the byte-weight of your
 * entrypoint modules including, HTML, CSS, and JavaScript. This method
 * will also return an unresolvable paths encountered during the analysis
 *
 * ```typescript
 * import { analyzeCriticalPath } from "@bolte/critical-path";
 *
 * test("Critical Path should never exceed N bytes", async () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   const { htmlWeight, cssWeight, javascriptWeight } = await analyzeCriticalPath(buildPath);
 *   expect(htmlWeight).toBeLessThan(51200);
 *   expect(cssWeight).toBeLessThan(102400);
 *   expect(javascriptWeight).toBeLessThan(204800);
 * });
 * ```
 */
export async function analyzeCriticalPath(path: string) {
  const { stdout, stderr } = await withStdio(`${path} -j`);
  try {
    const result = stdout.trim();
    if (!result.length) {
      throw new Error("parse error");
    }
    const json = JSON.parse(result) as CriticalPathAnalysis;
    return json;
  } catch {
    console.error(stderr);
    throw stderr;
  }
}

/**
 * assertCriticalPath
 *
 * Returns true if the combined weight of critical HTML, CSS, and JS
 * does not exceed the input threshold bytes.
 *
 * ```typescript
 * import { assertCriticalPath } from "@bolte/critical-path";
 *
 * test("Critical Path should never exceed N bytes", async () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(await assertCriticalPath(buildPath, 204800)).toEqual(true);
 * });
 * ```
 */
export async function assertCriticalPath(path: string, bytes: number) {
  return (await measureCriticalPath(path)) < bytes;
}

/**
 * assertCriticalHTML
 *
 * Returns true if the weight of critical HTML does not exceed the input threshold bytes.
 *
 * ```typescript
 * import { assertCriticalHTML } from "@bolte/critical-path";
 *
 * test("Critical HTML should never exceed N bytes", async () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(await assertCriticalHTML(buildPath, 51200)).toEqual(true);
 * });
 * ```
 */
export async function assertCriticalHTML(path: string, bytes: number) {
  const results = await analyzeCriticalPath(path);
  return results.htmlWeight < bytes;
}

/**
 * assertCriticalCSS
 *
 * Returns true if the weight of critical CSS does not exceed the input threshold bytes.
 *
 * ```typescript
 * import { assertCriticalCSS } from "@bolte/critical-path";
 *
 * test("Critical CSS should never exceed N bytes", async () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(await assertCriticalCSS(buildPath, 102400)).toEqual(true);
 * });
 * ```
 */
export async function assertCriticalCSS(path: string, bytes: number) {
  const results = await analyzeCriticalPath(path);
  return results.cssWeight < bytes;
}

/**
 * assertCriticalJavaScript
 *
 * Returns true if the combined weight of critical JS does not exceed the input threshold bytes.
 *
 * ```typescript
 * import { assertCriticalJavaScript } from "@bolte/critical-path";
 *
 * test("Critical CSS should never exceed N bytes", async () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(await assertCriticalJavaScript(buildPath, 204800)).toEqual(true);
 * });
 * ```
 */
export async function assertCriticalJavaScript(path: string, bytes: number) {
  const results = await analyzeCriticalPath(path);
  return results.javascriptWeight < bytes;
}

/**
 * measureCriticalPath
 *
 * Returns the combined weight of critical HTML, CSS, and JavaScript in bytes
 *
 * ```typescript
 * import { measureCriticalPath } from "@bolte/critical-path";
 *
 * test("Critical CSS should never exceed N bytes", async () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(await measureCriticalPath(buildPath)).toBeLessThan(204800);
 * });
 * ```
 */
export async function measureCriticalPath(path: string) {
  const results = await analyzeCriticalPath(path);
  return results.htmlWeight + results.cssWeight + results.javascriptWeight;
}

/**
 * cli
 *
 * Runs the critical path check as a CLI command logging all results
 * to `stdout`
 *
 * ```typescript
 * import { cli } from "@bolte/critical-path";
 *
 * cli("/path/to/index.html");
 * ```
 */
export function cli(path: string) {
  return new ChildProcess(`critical-path-check ${path}`).handler;
}

function withStdio(args: string) {
  let stdout: string[] = [];
  let stderr: string[] = [];
  return new Promise<{ stdout: string; stderr: string }>(resolve => {
    const CP = spawn("critical-path-check", args.split(" "), {
      stdio: ["pipe"],
    });
    CP.stdout?.on?.("data", data => stdout.push(data?.toString?.() ?? ""));
    CP.stderr?.on?.("data", data => stderr.push(data?.toString?.() ?? ""));
    CP.on("exit", () => {
      resolve({ stdout: stdout.join(""), stderr: stderr.join("") });
    });
  });
}
