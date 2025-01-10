# :eyes: `watch-files`

`watch-files` is a basic file system watcher to execute a command upon file changes.

## Install

To install the binary, run:

```shell
cargo install watch-files
```

## Usage 

```
  <files> | watch-files "<command>"
  <files> | watch-files [-p <poll_time_in_ms>] "<command>"
  watch-files -h|--help
```

## Examples

The following will watch all files in the `src` directory, and run `cargo build` when a file changes.

```shell
find src/* | watch-files "cargo build"
```

The following will watch all files in the `src` directory every `1000` milliseconds, and run `cargo build` when a file change is detected.

```shell
find src/* | watch-files --poll-rate 1000 "cargo build"
```

The following will print the help text

```shell
watch-files --help
```