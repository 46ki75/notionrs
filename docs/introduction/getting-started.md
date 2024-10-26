# Getting Started

The notionrs crate can be added in the same way as other common crates. The following steps outline the general process.

## Installation

To add this crate, use the following command:

```sh
cargo add notionrs
```

Alternatively, add the following section to your `Cargo.toml` file:

```toml
[dependencies]
notionrs = "1"
```

::: tip

To search the latest version of this crate, use the following command:

```sh
cargo search notionrs
```

Alternatively, you can find more detailed information on [crates.io](https://crates.io/crates/notionrs).

:::

## Usage with Async Runtime

The `notionrs` crate is asynchronous by design, so you will need to use it within an async runtime. While you have the freedom to choose any async runtime (such as [tokio](https://tokio.rs) or [async-std](https://async.rs)), here we provide an example using `tokio`, which is one of the most popular choices in the Rust ecosystem.

To use `tokio`, you can add it to your project with the following command:

```sh
cargo add tokio --features full
```

Alternatively, you can add it manually to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
notionrs = "1"
```

::: info

You can also use other async runtimes like `async-std`. Just make sure to adapt the runtime setup according to the runtime you choose. However, the examples in this documentation will assume you are using `tokio`.

:::
