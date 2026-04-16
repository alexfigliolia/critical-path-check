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
  function analyzeCriticalPath(path: string): CriticalPathAnalysis;
  function assertCriticalPath(path: string, bytes: number): boolean;
  function assertHTML(path: string, bytes: number): boolean;
  function assertCSS(path: string, bytes: number): boolean;
  function assertJavaScript(path: string, bytes: number): boolean;
  function measureCriticalPath(path: string): number;
}

export function analyzeCriticalPath(path: string): CriticalPathAnalysis {
  return addon.analyzeCriticalPath(path);
}

export function assertCriticalPath(path: string, bytes: number): boolean {
  return addon.assertCriticalPath(path, bytes);
}

export function assertHTML(path: string, bytes: number): boolean {
  return addon.assertHTML(path, bytes);
}

export function assertCSS(path: string, bytes: number): boolean {
  return addon.assertCSS(path, bytes);
}

export function assertJavaScript(path: string, bytes: number): boolean {
  return addon.assertJavaScript(path, bytes);
}

export function measureCriticalPath(path: string): number {
  return addon.measureCriticalPath(path);
}
