import { cwd } from "node:process";
import { join } from "node:path";

import test from "ava";

import { analyzeCriticalPath } from "../externals/index";

test("test with errors encountered", async t => {
  const result = await analyzeCriticalPath(
    join(cwd(), "test-with-errors/index.html"),
  );
  t.is(Object.keys(result.unresolvedPaths).length, 3);
});

test("assertions on deterministic values", async t => {
  const result = await analyzeCriticalPath(
    join(cwd(), "test-with-errors/index.html"),
  );
  t.is(result.analysis.htmlWeight, 542);
  t.is(result.analysis.cssWeight, 4371);
  t.is(result.analysis.javascriptWeight, 611997);
});
