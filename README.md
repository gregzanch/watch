# :eyes: Watch

`watch` is a basic file system watcher to execute a command upon file changes.

## Usage 

```
  <files> | watch "<command>"
  <files> | watch [-p <poll_time_in_ms>] "<command>"
  watch -h|--help
```

## Examples

The following will watch all files in the `src` directory, and run `cargo build` when a file changes.

```shell
find src/* | watch "cargo build"
```

The following will watch all files in the `src` directory every `1000` milliseconds, and run `cargo build` when a file change is detected.

```shell
find src/* | watch --poll-rate 1000 "cargo build"
```

The following will print the help text

```shell
watch --help
```