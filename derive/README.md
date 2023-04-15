These proc macros help you to implement the `envir::Serialize` and
`envir::Deserialize` traits.

# Attributes

By default, these macro use the uppercase field name as environment variable
name.

```rust
use envir::Deserialize;

#[derive(envir::Deserialize, Debug)]
struct Config {
    home: String,
}

let config = Config::from_env();
dbg!(config);
```

```bash
$ cargo run
[src/main.rs:12] config = Ok(
    Config {
        home: "/home/sanpi",
    }
)
```

## Container

- `prefix`: sets this attributes to add this prefix at the field name.

```rust
use envir::Deserialize;

#[derive(envir::Deserialize, Debug)]
#[envir(prefix = "APP_")]
struct Config {
    dir: String,
}

let config = Config::from_env();
dbg!(config);
```

```bash
$ export APP_DIR=~/.config/app
$ cargo run
[src/main.rs:12] config = Ok(
    Config {
        dir: "/home/sanpi/.config/app",
    }
)
```

## Field

- `name`: use this name for the environment variable instead of the name of the
    field. If `prefix` is defined, it also prepend to this name;
- `export_with`: use this function to export this field. The given function must
    be callable as `fn (T) -> HashMap<String, String>`;
- `load_with`: use this function to load this field. The given function must
    be callable as `fn (Hashmap<String, String>) -> envir::Result<T>`;
- `noprefix`: doesn’t add the `prefix` for this field;
- `nested`: this field should be de/serialized recursively.

```rust
use envir::Deserialize;

#[derive(envir::Deserialize, Debug)]
#[envir(prefix = "APP_")]
struct Config {
    dir: String,
}

let config = Config::from_env();
dbg!(config);
```

```bash
$ export APP_DIR=~/.config/app
$ cargo run
[src/main.rs:12] config = Ok(
    Config {
        dir: "/home/sanpi/.config/app",
    }
)
```
