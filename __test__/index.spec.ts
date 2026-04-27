import { cwd } from "node:process";
import { join } from "node:path";

import test from "ava";

import {
  analyzeCriticalPath,
  cli,
  measureCriticalPath,
  assertCriticalCSS,
  assertCriticalHTML,
  assertCriticalJavaScript,
  assertCriticalPath,
} from "../externals";

test("test with errors encountered", async t => {
  const result = await analyzeCriticalPath(
    join(cwd(), "fixtures/test-with-errors/index.html"),
  );
  t.is(Object.keys(result.unresolvedPaths).length, 3);
  t.is(result.htmlWeight, 542);
  t.is(result.cssWeight, 4454);
  t.is(result.javascriptWeight, 611996);
});

test("test with no errors encountered", async t => {
  const result = await analyzeCriticalPath(
    join(cwd(), "fixtures/test-portfolio/index.html"),
  );
  t.is(Object.keys(result.unresolvedPaths).length, 0);
  t.is(result.htmlWeight, 646);
  t.is(result.cssWeight, 11782);
  t.is(result.javascriptWeight, 1336333);
});

test("test cli", async t => {
  const result = await cli(join(cwd(), "fixtures/test-portfolio/index.html"));
  t.is(result, undefined);
});

test("test measure", async t => {
  t.is(
    await measureCriticalPath(
      join(cwd(), "fixtures/test-portfolio/index.html"),
    ),
    1348761,
  );
});

[
  assertCriticalPath,
  assertCriticalCSS,
  assertCriticalHTML,
  assertCriticalJavaScript,
].forEach(method => {
  test(`test assertions ${method.name}`, async t => {
    t.is(
      await method(join(cwd(), "fixtures/test-portfolio/index.html"), 100),
      false,
    );
    t.is(
      await method(join(cwd(), "fixtures/test-portfolio/index.html"), 10000000),
      true,
    );
  });
});

test("test throwing stderr", async t => {
  let result: string = "";
  try {
    await analyzeCriticalPath(
      join(cwd(), "fixtures/test-with-fake-path/index.html"),
    );
  } catch (error: unknown) {
    if (typeof error === "string") {
      result = error;
    }
  }
  t.regex(
    result as unknown as string,
    /The specified input path does not exis/gm,
  );
});
