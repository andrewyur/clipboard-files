# Clipboard Files

This crate lets you read and write file paths to/from the system wide clipboard, that are copied from and can be used by Explorer, Finder, etc.

It's supported on Windows, Linux (using GTK) and MacOS.

## Reading

```rust
use clipboard_files;
use std::path::PathBuf;

fn main() {
    let file_path = PathBuf::from(file!());
    clipboard_files::write(vec![file_path]).unwrap();

    let files = clipboard_files::read();
    println!(files);
}
```

## Why?

There are several clipboard crates, for instance https://github.com/1Password/arboard.
That crate is supported in multiple unix-like environments because it talks X11 directly.
This crate uses the GTK bindings for Linux, which offers a much simpler API.

Ideally, all upstream crates should support files. When they do, we'd be better off deleting
this one. In the meantime, use this crate.

## License

MIT OR Apache-2.0.