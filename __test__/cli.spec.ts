import { cwd } from "node:process";
import { join } from "node:path";

import test from "ava";

import { cli } from "../dist";

test("test cli", t => {
  const result = cli(join(cwd(), "fixtures/test-portfolio/index.html"));
  t.is(result, undefined);
});
