import * as addon from "./load";

interface CriticalPathAnalysis {
  htmlWeight: number;
  javascriptWeight: number;
  cssWeight: number;
  unresolvedPaths: Record<string, string[]>;
}

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.js" {
  function cli(path: string): void;
  function analyzeCriticalPath(path: string): CriticalPathAnalysis;
  function assertCriticalPath(path: string, bytes: number): boolean;
  function assertCriticalHTML(path: string, bytes: number): boolean;
  function assertCriticalCSS(path: string, bytes: number): boolean;
  function assertCriticalJavaScript(path: string, bytes: number): boolean;
  function measureCriticalPath(path: string): number;
}

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
 * test("Critical Path should never exceed N bytes", () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   const { htmlWeight, cssWeight, javascriptWeight } = analyzeCriticalPath(buildPath);
 *   expect(htmlWeight).toBeLessThan(51200);
 *   expect(cssWeight).toBeLessThan(102400);
 *   expect(javascriptWeight).toBeLessThan(204800);
 * });
 * ```
 */
export function analyzeCriticalPath(path: string): CriticalPathAnalysis {
  return addon.analyzeCriticalPath(path);
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
 * test("Critical Path should never exceed N bytes", () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(assertCriticalPath(buildPath, 204800)).toEqual(true);
 * });
 * ```
 */
export function assertCriticalPath(path: string, bytes: number): boolean {
  return addon.assertCriticalPath(path, bytes);
}

/**
 * assertCriticalHTML
 *
 * Returns true if the weight of critical HTML does not exceed the input threshold bytes.
 *
 * ```typescript
 * import { assertCriticalHTML } from "@bolte/critical-path";
 *
 * test("Critical HTML should never exceed N bytes", () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(assertCriticalHTML(buildPath, 51200)).toEqual(true);
 * });
 * ```
 */
export function assertCriticalHTML(path: string, bytes: number): boolean {
  return addon.assertCriticalHTML(path, bytes);
}

/**
 * assertCriticalCSS
 *
 * Returns true if the weight of critical CSS does not exceed the input threshold bytes.
 *
 * ```typescript
 * import { assertCriticalCSS } from "@bolte/critical-path";
 *
 * test("Critical CSS should never exceed N bytes", () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(assertCriticalCSS(buildPath, 102400)).toEqual(true);
 * });
 * ```
 */
export function assertCriticalCSS(path: string, bytes: number): boolean {
  return addon.assertCriticalCSS(path, bytes);
}

/**
 * assertCriticalJavaScript
 *
 * Returns true if the combined weight of critical JS does not exceed the input threshold bytes.
 *
 * ```typescript
 * import { assertCriticalJavaScript } from "@bolte/critical-path";
 *
 * test("Critical CSS should never exceed N bytes", () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(assertCriticalJavaScript(buildPath, 204800)).toEqual(true);
 * });
 * ```
 */
export function assertCriticalJavaScript(path: string, bytes: number): boolean {
  return addon.assertCriticalJavaScript(path, bytes);
}

/**
 * measureCriticalPath
 *
 * Returns the combined weight of critical HTML, CSS, and JavaScript in bytes
 *
 * ```typescript
 * import { measureCriticalPath } from "@bolte/critical-path";
 *
 * test("Critical CSS should never exceed N bytes", () => {
 *   const buildPath = path.join(process.cwd(), "dist", "index.html");
 *   expect(measureCriticalPath(buildPath)).toBeLessThan(204800);
 * });
 * ```
 */
export function measureCriticalPath(path: string): number {
  return addon.measureCriticalPath(path);
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
  return addon.cli(path);
}
