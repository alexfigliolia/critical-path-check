import { cwd } from "node:process";
import { join } from "node:path";

import { Bench } from "tinybench";

import { measureCriticalPath } from "../externals";

void (async () => {
  const b = new Bench();

  b.add("Benchmark a test build with errors", async () => {
    measureCriticalPath(join(cwd(), "test-with-errors/index.html"));
  });

  b.add("Benchmark test with portfolio build", async () => {
    measureCriticalPath(join(cwd(), "test-portfolio/index.html"));
  });

  await b.run();

  console.table(b.table());
})();
