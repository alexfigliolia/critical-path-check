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

test("test with errors encountered", t => {
  const result = analyzeCriticalPath(
    join(cwd(), "fixtures/test-with-errors/index.html"),
  );
  t.is(Object.keys(result.unresolvedPaths).length, 3);
  t.is(inByteRange(result.htmlWeight, 542), true);
  t.is(inByteRange(result.cssWeight, 4454), true);
  t.is(inByteRange(result.javascriptWeight, 611996), true);
});

test("test with no errors encountered", t => {
  const result = analyzeCriticalPath(
    join(cwd(), "fixtures/test-portfolio/index.html"),
  );
  t.is(Object.keys(result.unresolvedPaths).length, 0);
  t.is(inByteRange(result.htmlWeight, 646), true);
  t.is(inByteRange(result.cssWeight, 11782), true);
  t.is(inByteRange(result.javascriptWeight, 1336333), true);
});

test("test measure", t => {
  t.is(
    inByteRange(
      measureCriticalPath(join(cwd(), "fixtures/test-portfolio/index.html")),
      1348761,
    ),
    true,
  );
});

[
  assertCriticalPath,
  assertCriticalCss,
  assertCriticalHtml,
  assertCriticalJavaScript,
].forEach(method => {
  test(`test assertions ${method.name}`, t => {
    t.is(method(join(cwd(), "fixtures/test-portfolio/index.html"), 100), false);
    t.is(
      method(join(cwd(), "fixtures/test-portfolio/index.html"), 10000000),
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
  test(`test throwing stderr - ${message}`, t => {
    t.throws(() => analyzeCriticalPath(path), { message });
  });
});

function inByteRange(input: number, target: number, allowance: number = 0.1) {
  const allowanceBytes = target * allowance;
  const min = target - allowanceBytes;
  const max = target + allowanceBytes;
  console.log("input: ", input);
  console.log(`Range: ${min} - ${max}`);
  return input >= min && input <= max;
}
