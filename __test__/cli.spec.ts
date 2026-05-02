import { cwd } from "node:process";
import { join } from "node:path";

import test from "ava";

import { cli } from "../dist";

test("test cli", t => {
  t.is(
    t.notThrows(() => cli(join(cwd(), "fixtures/test-portfolio/index.html"))),
    true,
  );
});

test("test json", t => {
  t.is(
    t.notThrows(() =>
      cli(join(cwd(), "fixtures/test-portfolio/index.html"), true),
    ),
    true,
  );
});
