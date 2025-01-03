# envir

[![Github actions](https://github.com/sanpii/envir/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/envir/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Gitlab CI](https://gitlab.com/sanpi/envir/badges/main/pipeline.svg)](https://gitlab.com/sanpi/envir/commits/main)

A toolbox to deal with your environment.

## Basics

Without feature, this crate provides simple functions to retreive the value of
an environment variable:

- `get` to retreive as string;
- `parse` to directly parse the value as a desired type.

The `try_` version of these functions return `None` if the variable doens’t
exist when `get` and `parse` return the `Error::Missing` error.

In addition this crate provide a `set` function, like `std::env::set_var` but
works for all types implement `ToString`.

Finally, a `collect` function to retreive all environment variables in a easy to
print collection.

## dotenv

The `dotenv` feature adds an eponyme function to load `.env` file.

## logger

The `logger` feature adds logger configured via environment variables.

## serde

The `serde` feature adds macro to deserialize struct from env:

```rust,ignore
use envir::Deserialize;

#[derive(envir::Deserialize)]
struct Config {
}

fn main() -> envir::Result {
    let config: Config = envir::from_env()?;
    // or
    let config = Config::from_env()?;

    Ok(())
}
```

And serialize to env:

```rust,ignore
use envir::Serialize;

#[derive(envir::Serialize, Default)]
struct Config {
}

let config = Config::default();
config.export();
```

The `extrapolation` feature allows environment variables replacement in the
default macro attribute:

```rust,ignore
#[derive(envir::Deserialize)]
struct Config {
    #[envir(default = "/home/${USER}")]
    home: String,
}
```

You can read the [envir_derive crate
documentation](https://docs.rs/envir_derive/) for more informations.
