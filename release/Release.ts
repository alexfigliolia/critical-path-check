import { createInterface } from "node:readline/promises";
import { join } from "node:path";
import { readdir, writeFile } from "node:fs/promises";
import { createReadStream, existsSync } from "node:fs";

import { Logger, SemverRelease } from "@figliolia/semver";
import { ChildProcess } from "@figliolia/child-process";

export class Release extends SemverRelease {
  private static readonly CRATES_PATH = join(this.ROOT, "internals");
  private static readonly LOCAL_CRATE_DEPENDENCY = /^.*= {.*path = "..\/.*}/;
  private static readonly LOCAL_CRATE_VERSION_REPLACER =
    /^.*= {.*version = "(.*?)".*}/;
  constructor() {
    super({
      onComplete: async version => {
        await Release.writeCargoVersions(version);
        Logger.info("Linting Everything...");
        await new ChildProcess("pnpm lint:ts").handler;
        await new ChildProcess("pnpm lint:rust").handler;
        Logger.info("Compiling for production...");
        await new ChildProcess("pnpm build:rust").handler;
        await new ChildProcess("pnpm build").handler;
      },
    });
  }

  private static async writeCargoVersions(version: string) {
    const tasks: Promise<void>[] = [];
    const entries = await readdir(this.CRATES_PATH, { withFileTypes: true });
    for (const entry of entries) {
      if (entry.isDirectory()) {
        const cargoPath = join(this.CRATES_PATH, entry.name, "Cargo.toml");
        if (existsSync(cargoPath)) {
          tasks.push(this.modCargoFile(cargoPath, version));
        }
      }
    }
    return Promise.all(tasks);
  }

  private static async modCargoFile(path: string, version: string) {
    let writeVersion = true;
    let writeLocalDependencies = true;
    const content = await this.streamFileContent(path, line => {
      if (writeVersion && line.startsWith('version = "')) {
        writeVersion = false;
        return `version = "${version}"`;
      }
      if (writeLocalDependencies && this.LOCAL_CRATE_DEPENDENCY.test(line)) {
        writeLocalDependencies = false;
        return line.replace(this.LOCAL_CRATE_VERSION_REPLACER, (match, p1) => {
          return match.replace(p1, version);
        });
      }
      return line;
    });
    await writeFile(path, content);
  }

  private static async streamFileContent(
    path: string,
    onLine: (line: string) => string,
  ) {
    const reader = createInterface({
      input: createReadStream(path),
      crlfDelay: Infinity,
    });
    const lines: string[] = [];
    for await (const line of reader) {
      lines.push(onLine(line));
    }
    reader.close();
    return lines.join("\n");
  }
}
