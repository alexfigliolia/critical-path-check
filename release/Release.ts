import { createInterface } from "node:readline/promises";
import { join } from "node:path";
import { writeFile } from "node:fs/promises";
import { createReadStream } from "node:fs";

import { Logger, SemverRelease } from "@figliolia/semver";
import { ChildProcess } from "@figliolia/child-process";

export class Release extends SemverRelease {
  private static readonly WORKSPACES = join(this.ROOT, "internals");
  private static readonly CARGO_FILE_PATHS = ["core", "wasm"].map(pkg =>
    join(this.WORKSPACES, pkg, "Cargo.toml"),
  );
  constructor() {
    super({
      onComplete: async version => {
        await Release.writeCargoVersions(version);
        Logger.info("Linting Everything...");
        await new ChildProcess("yarn lint:ts").handler;
        await new ChildProcess("yarn lint:rust").handler;
      },
    });
  }

  private static async writeCargoVersions(version: string) {
    return Promise.all(
      this.CARGO_FILE_PATHS.map(path =>
        (async () => {
          let write = true;
          const content = await this.streamFileContent(path, line => {
            if (write && line.startsWith('version = "')) {
              write = false;
              return `version = "${version}"`;
            }
            return line;
          });
          await writeFile(path, content);
        })(),
      ),
    );
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
