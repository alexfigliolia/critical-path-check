import { cwd } from "node:process";
import { join } from "node:path";

import { Bench } from "tinybench";

import { analyzeCriticalPath } from "../externals/index.js";

const b = new Bench();

b.add("Benchmark a test build with errors", async () => {
  await analyzeCriticalPath(join(cwd(), "test-with-errors/index.html"));
});

b.add("Benchmark test with portfolio build", async () => {
  await analyzeCriticalPath(join(cwd(), "test-portfolio/index.html"));
});

await b.run();

console.table(b.table());
