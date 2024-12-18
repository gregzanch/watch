# Watch

`watch` is a basic file system watcher to execute a command upon file changes.

## Usage 

The following will watch all files in the `src` directory, and run `cargo build` when a file change.

```sh
find src/* | watch -e "cargo build"
```