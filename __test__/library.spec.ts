import { cwd } from "node:process";
import { join } from "node:path";

import test from "ava";

import {
  analyzeCriticalPath,
  measureCriticalPath,
  assertCriticalCss,
  assertCriticalHtml,
  assertCriticalJavaScript,
  assertCriticalPath,
} from "../dist";

test("test with errors encountered", async t => {
  const result = await analyzeCriticalPath(
    join(cwd(), "fixtures/test-with-errors/index.html"),
  );
  console.log(result.resolvedPaths);
  console.log(result.unresolvedPaths);
  t.is(result.htmlWeight, 542);
  t.is(result.cssWeight, 4454);
  t.is(result.javascriptWeight, 611996);
  t.not(Object.keys(result.resolvedPaths).length, 0);
  t.is(Object.keys(result.unresolvedPaths).length, 3);
});

test("test with no errors encountered", async t => {
  const result = await analyzeCriticalPath(
    join(cwd(), "fixtures/test-portfolio/index.html"),
  );
  console.log(result.resolvedPaths);
  console.log(result.unresolvedPaths);
  t.is(result.htmlWeight, 646);
  t.is(result.cssWeight, 11782);
  t.is(result.javascriptWeight, 1336333);
  t.not(Object.keys(result.resolvedPaths).length, 0);
  t.is(Object.keys(result.unresolvedPaths).length, 0);
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
  assertCriticalCss,
  assertCriticalHtml,
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

[
  [
    join(cwd(), "fixtures/test-with-fake-path/index.html"),
    "CP Check: The specified input path does not exist",
  ],
  [
    join(cwd(), "fixtures/test-with-errors/assets"),
    "CP Check: The specified input does not point to an html file",
  ],
  [
    "./fixtures/test-with-errors/assets",
    "CP Check: Your input path must be an absolute path to your root HTML file",
  ],
  [
    "https://not-an-actual-domain@#($&(*#@$.com",
    "CP Check: Failed to parse the root HTML file",
  ],
].forEach(([path, message]) => {
  test(`test throwing stderr - ${message}`, async t => {
    await t.throwsAsync(() => analyzeCriticalPath(path), { message });
  });
});
