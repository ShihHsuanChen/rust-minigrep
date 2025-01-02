# Minigrep

A simple CLI for searching pattern in a text file.

## Requirements

- `rustc ~= 1.83.0`
- `cargo ~= 1.83.0`

## Dependencies

- `regex == 1.11.1`

## Usage

```
minigrep [OPTIONS] <query> <file_path>

Options:
  -h, --help         print this help message.

Positional:
  query     (string) regular expression pattern.
  file_path (string) path to the file to search.
```

## Examples (using `Cargo run`)
### print help message

```
$ cargo run -- --help
```

### search

```
$ cargo run -- "you\W" peom.txt
1:I'm nobody! Who are you?
2:Are you nobody, too?
4:They'd banish us, you know.
```

## Build for release

```
$ cargo build --release
```

After finish, you can try the executable

```
$ ./target/release/minigrep "you\W" peom.txt
1:I'm nobody! Who are you?
2:Are you nobody, too?
4:They'd banish us, you know.
```
