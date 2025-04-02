This is a [Rust](https://www.rust-lang.org/) project bootstrapped build on [Ratzilla](https://orhun.dev/ratzilla/) and [Ratatui](https://ratatui.rs/)

A terminal-esque personal page that has a colorful implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) overlaid atop.

## [Getting Started](https://github.com/orhun/ratzilla#serve)

Install [trunk](https://trunkrs.dev) to build and serve the web application.

```shell
cargo install --locked trunk
```

Add compilation target `wasm32-unknown-unknown`:

```shell
rustup target add wasm32-unknown-unknown
```

Then serve it on your browser:

```shell
trunk serve
```

Now go to [http://localhost:8080](http://localhost:8080) and enjoy TUIs in your browser!

## Learn More

To learn more about Rust, Ratzilla, and Ratatui, take a look at the following resources:

- [Rust Programming Language](https://www.rust-lang.org/) - A language empowering everyone to build reliable and efficient software
- [Ratatui](https://ratatui.rs/) - A Rust crate for cooking up Terminal User Interfaces
- [Ratzilla](https://orhun.dev/ratzilla/) - Build terminal-themed web applications with Rust and WebAssembly

## Deploy on Vercel

The easiest way to deploy this onto Vercel is to click:
[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https%3A%2F%2Fgithub.com%2Fjetpham%2Fjetpham-com)
