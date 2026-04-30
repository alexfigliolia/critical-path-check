# Critical Path Check

A rust powered tool for making assertions on a web application's critical path

1. [Installation](#installation)
2. [JavaScript API](#javascript-api)
3. [Rust API](#rust-api)
4. [Command Line](#command-line)

## Installation

#### JavaScript/TypeScript

```bash
npm i -D @ui-perf/critical-path
yarn add -D @ui-perf/critical-path
pnpm add -D @ui-perf/critical-path
```

#### Rust

```bash
cargo add critical_path_check
# or
cargo install critical_path_check
```

### JavaScript API

All methods exposed by the JavaScript API require a path to an HTML file. This path can be

1. An absolute path to an HTML file locally on your machine
2. The URL of a remote HTML file - ie - `https://github.com`

#### `analyzeCriticalPath`

Returns a critical path analysis containing the byte-weight of your
entrypoint modules including, HTML, CSS, and JavaScript. This method
will also return any unresolvable paths encountered during the analysis

```typescript
import { analyzeCriticalPath } from "@ui-perf/critical-path";

test("Critical Path should never exceed N bytes", () => {
  const buildPath = path.join(process.cwd(), "dist", "index.html");
  const { htmlWeight, cssWeight, javascriptWeight } =
    analyzeCriticalPath(buildPath);
  expect(htmlWeight).toBeLessThan(51200);
  expect(cssWeight).toBeLessThan(102400);
  expect(javascriptWeight).toBeLessThan(204800);
});
```

#### `measureCriticalPath`

Returns the combined weight of critical HTML, CSS, and JavaScript in bytes

```typescript
import { measureCriticalPath } from "@ui-perf/critical-path";

test("The critical path should never exceed N bytes", () => {
  const buildPath = path.join(process.cwd(), "dist", "index.html");
  expect(measureCriticalPath(buildPath)).toBeLessThan(204800);
});
```

#### `assertCriticalPath`

Returns true if the combined weight of critical HTML, CSS, and JS
does not exceed the input threshold bytes.

```typescript
import { assertCriticalPath } from "@ui-perf/critical-path";

test("Critical Path should never exceed N bytes", () => {
  const buildPath = path.join(process.cwd(), "dist", "index.html");
  expect(assertCriticalPath(buildPath, 204800)).toEqual(true);
});
```

#### `assertCriticalHTML`

Returns true if the weight of critical HTML does not exceed the input threshold bytes.

```typescript
import { assertCriticalHTML } from "@ui-perf/critical-path";

test("Critical HTML should never exceed N bytes", () => {
  const buildPath = path.join(process.cwd(), "dist", "index.html");
  expect(assertCriticalHTML(buildPath, 51200)).toEqual(true);
});
```

#### `assertCriticalCSS`

Returns true if the weight of critical CSS does not exceed the input threshold bytes.

```typescript
import { assertCriticalCSS } from "@ui-perf/critical-path";

test("Critical CSS should never exceed N bytes", () => {
  const buildPath = path.join(process.cwd(), "dist", "index.html");
  expect(assertCriticalCSS(buildPath, 102400)).toEqual(true);
});
```

#### `assertCriticalJavaScript`

Returns true if the combined weight of critical JS does not exceed the input threshold bytes.

```typescript
import { assertCriticalJavaScript } from "@ui-perf/critical-path";

test("Critical JavaScript should never exceed N bytes", () => {
  const buildPath = path.join(process.cwd(), "dist", "index.html");
  expect(assertCriticalJavaScript(buildPath, 204800)).toEqual(true);
});
```

### Rust API

#### `analyze_critical_path`

Analyzes the target HTML file's critical render path

Returns the critical weight (in bytes) required to render your page. It also returns any unresolvable paths that were encountered during the analysis

```rust
use critical_path_check::analyze_critical_path;

let my_html = PathBuf::from("/path/to/my/root.html");
let result = analyze_critical_path(&my_html);

println!("Total JS Bytes: {}", result.analysis.javascript_weight);
println!("Total CSS Bytes: {}", result.analysis.css_weight);
println!("Total HTML Bytes: {}", result.analysis.html_weight);
```

#### `CriticalPathCheck`

The underlying `struct` and `impl` powering the critical path check.

There are three ways to spawn instances of the `CriticalPathCheck`

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

/// using a string representing an absolute path to an HTML file
let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
/// or using a URL to a remotely hosted build
let cp_check = CriticalPathCheck::new("https://my-app.com");
/// or using a PathBuf
let cp_check = CriticalPathCheck::from(PathBuf::from("/path/to/my/root.html"));
```

#### `CriticalPathCheck.run(&self): CriticalPathAnalysis`

Returns a critical path analysis containing the byte-weights of critical HTML, CSS, and JavaScript as well as any unresolvable imports/references encountered

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
let result = cp_check.run();
```

#### `CriticalPathCheck.measure(&self): usize`

Returns the combined weight of critical HTML, CSS, and JavaScript

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
let total_bytes = cp_check.measure();
```

#### `CriticalPathCheck.assert(&self, bytes: usize): bool`

Returns true if the specified number of bytes is greater than the cummulative critical path

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
let check_passed = cp_check.assert(1000000);
```

#### `CriticalPathCheck.assert_html(&self, bytes: usize): bool`

Returns true if the specified number of bytes is greater than the byte-weight of the critical HTML

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
let check_passed = cp_check.assert_html(50000);
```

#### `CriticalPathCheck.assert_css(&self, bytes: usize): bool`

Returns true if the specified number of bytes is greater than the byte-weight of the critical CSS

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
let check_passed = cp_check.assert_css(100000);
```

#### `CriticalPathCheck.assert_javascript(&self, bytes: usize): bool`

Returns true if the specified number of bytes is greater than the byte-weight of the critical JavaScript

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
let check_passed = cp_check.assert_javascript(500000);
```

#### `CriticalPathCheck.run_cli(&self)`

Executes the critical path analysis as a CLI command logging all results to `stdout`

```rust
use critical_path_check::critical_path_check::CriticalPathCheck;

let cp_check = CriticalPathCheck::new("/path/to/my/root.html");
cp_check.run_cli();
```

### Command Line

The critical path check can be used as a CLI simply by installing the crate and running

```bash
# cargo install critical-path-check
critical-path-check /absolute/path/or/url/to/your-app.html
```
