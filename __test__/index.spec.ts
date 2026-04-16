import { cwd } from "node:process";
import { join } from "node:path";

import test from "ava";

import { analyzeCriticalPath } from "../lib/index.mjs";

test("test with errors encountered", t => {
  const result = analyzeCriticalPath(
    join(cwd(), "test-with-errors/index.html"),
  );
  t.is(Object.keys(result.unresolvedPaths).length, 3);
  t.is(result.htmlWeight, 542);
  t.is(result.cssWeight, 4371);
  t.is(result.javascriptWeight, 611997);
});

test("test with no errors encountered", t => {
  const result = analyzeCriticalPath(join(cwd(), "test-portfolio/index.html"));
  t.is(Object.keys(result.unresolvedPaths).length, 0);
  t.is(result.htmlWeight, 646);
  t.is(result.cssWeight, 11782);
  t.is(result.javascriptWeight, 1336333);
});
