export interface CriticalPathAnalysis {
  htmlWeight: number;
  javascriptWeight: number;
  cssWeight: number;
  unresolvedPaths: Record<string, string[]>;
}
