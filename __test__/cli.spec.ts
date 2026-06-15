import { cwd } from "node:process";
import { join } from "node:path";

import test from "ava";

import { cli } from "../dist";

test("test cli", async t => {
  await t.notThrowsAsync(() =>
    cli(join(cwd(), "fixtures/test-portfolio/index.html")),
  );
});

test("test json", async t => {
  await t.notThrowsAsync(() =>
    cli(join(cwd(), "fixtures/test-portfolio/index.html"), true),
  );
});
