# `rgrep` — Project Specification

Build a command-line file search utility inspired by `grep`, intentionally scoped as a Rust learning project.

The goal is not feature parity with GNU `grep`. The goal is to encounter core Rust concepts through a realistic CLI tool.

## Core Command

```bash
rgrep <pattern> <path>
```

Examples:

```bash
rgrep "TODO" src/main.rs
rgrep "error" logs/
rgrep "fn main" .
```

The command searches for the given pattern in a file or recursively through a directory.

## Functional Requirements

### 1. Search a Single File

```bash
rgrep "hello" notes.txt
```

Print every matching line.

### 2. Search Directories Recursively

```bash
rgrep "TODO" src/
```

Traverse all nested directories and search regular files.

### 3. Print File Path and Line Number

Output format:

```text
src/main.rs:14:    // TODO: refactor this
src/lib.rs:27:     // TODO: handle errors
```

### 4. Case-Insensitive Search

```bash
rgrep -i "error" logs/
```

Should match:

```text
error
Error
ERROR
```

### 5. Invert Matches

```bash
rgrep -v "debug" app.log
```

Print lines that **do not** contain the pattern.

### 6. Count Matches

```bash
rgrep -c "TODO" src/
```

Example:

```text
src/main.rs:3
src/lib.rs:7
```

### 7. Limit File Types

```bash
rgrep --ext rs "unsafe" .
rgrep --ext py "TODO" .
```

### 8. Ignore Hidden Files and Directories by Default

For example:

```text
.git/
.env
.cache/
```

Provide an option to include them:

```bash
rgrep --hidden "foo" .
```

### 9. Graceful Error Handling

The program should not panic for normal user errors.

For example:

```bash
rgrep foo missing.txt
```

should produce something like:

```text
rgrep: missing.txt: No such file or directory
```

and exit with a non-zero status code.

### 10. Help Output

```bash
rgrep --help
```

Something approximately like:

```text
Usage: rgrep [OPTIONS] <PATTERN> <PATH>

Arguments:
  <PATTERN>  Pattern to search for
  <PATH>     File or directory to search

Options:
  -i              Case-insensitive search
  -v              Invert match
  -c              Count matching lines
  --ext <EXT>     Search only files with this extension
  --hidden        Include hidden files
  -h, --help      Print help
```

## Architecture Requirements

Don't put everything in `main.rs`.

Aim for roughly:

```text
src/
├── main.rs
├── cli.rs
├── search.rs
├── walker.rs
└── error.rs
```

The exact module boundaries are up to you.

Conceptually:

```text
CLI
 │
 ▼
Config
 │
 ▼
File discovery
 │
 ▼
File reading
 │
 ▼
Search
 │
 ▼
SearchResult
 │
 ▼
Output
```

I'd expect you to introduce a configuration type approximately like:

```rust
struct Config {
    pattern: String,
    path: PathBuf,
    ignore_case: bool,
    invert_match: bool,
    count: bool,
    extension: Option<String>,
    hidden: bool,
}
```

Don't treat that exact struct as prescribed design. Change it if you find a better representation.

## Search Result Model

Don't immediately print from deep inside the search logic.

Try to separate:

```text
finding matches
```

from:

```text
displaying matches
```

A result might conceptually contain:

```rust
struct Match {
    path: PathBuf,
    line_number: usize,
    line: String,
}
```

Again, decide yourself whether owning `String` here is appropriate. That question will become useful when you start dealing with borrowing.

## Error Handling

Avoid doing this throughout the application:

```rust
.unwrap()
```

`unwrap()` is fine while initially experimenting, but the finished version should handle expected errors using `Result`.

Your top-level flow should eventually look roughly like:

```rust
fn run(config: Config) -> Result<(), RgrepError>
```

with `main()` responsible for converting the result into a message and exit code.

You don't initially need a custom error type. Start simple and introduce one if the code starts benefiting from it.

## Testing Requirements

At minimum, test the search logic independently from the filesystem.

Given:

```text
Rust is fast
Python is productive
I like Rust
rust can be tricky
```

Searching for:

```text
Rust
```

should return lines:

```text
1
3
```

Case-insensitive search should return:

```text
1
3
4
```

Also test:

- no matches
- inverted matches
- empty input
- empty pattern
- Unicode text

Keep filesystem integration tests separate from pure search tests.

## Constraints

For the **first version**, use the Rust standard library as much as possible.

Don't immediately pull in libraries for everything.

In particular, initially implement the following yourself:

- argument parsing
- directory traversal
- file reading
- searching

Once the basic implementation works, replacing pieces with ecosystem crates becomes part of the learning exercise.

## Milestones

1. `rgrep <pattern> <file>` with hard-coded argument positions.
2. Extract the search operation into a function.
3. Add line numbers.
4. Introduce `Config`.
5. Add proper `Result`-based errors.
6. Support directories and recursive traversal.
7. Add `-i`.
8. Add `-v`.
9. Add `-c`.
10. Add extension filtering.
11. Add hidden-file handling.
12. Add unit and integration tests.
13. Replace your manual CLI parser with `clap`.
14. Compare your directory walker with the `walkdir` or `ignore` ecosystem.
15. Profile it on a large directory and identify obvious inefficiencies.

## Deliberately Out of Scope Initially

Do not implement these yet:

- regex
- colored output
- binary-file detection
- `.gitignore` support
- parallel searching
- memory mapping
- context lines (`-A` / `-B` / `-C`)
- multiple patterns
- stdin piping
- compressed files

These make good follow-up features.

Once the base version works, a useful progression is:

1. Regex support
2. `.gitignore` support
3. Parallel search

These upgrades will introduce more interesting Rust concepts after the fundamentals are in place.
