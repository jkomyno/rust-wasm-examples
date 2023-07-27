# Rust WebAssembly Examples

> Example usage of Rust/WebAssembly in Node.js, with TypeScript definitions automatically generated from Rust code whenever possible.

<p>
  <a href="https://github.com/jkomyno/rust-wasm-examples/actions/workflows/ci.yml">
    <img alt="Github Actions" src="https://github.com/jkomyno/rust-wasm-examples/actions/workflows/ci.yml/badge.svg?branch=main" target="_blank" />
  </a>

  <a href="https://github.com/jkomyno/rust-wasm-examples/blob/main/LICENSE">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" target="_blank" />
  </a>
  
</p>

## Disclaimer

This repository is an ongoing work in progress collecting examples from Alberto Schiabel's WebAssembly talks and experiments. For more complete repositories, please check out:
- https://github.com/jkomyno/grazjs-2023, containing code examples of "Rust to TypeScript" FFIs using `wasm-bingden`, `serde-wasm-bindgen`, and `tsify`
- https://github.com/jkomyno/rust-guild-trivago-2023, which contains more and interactive complex FFI examples + demos of how run I/O operations defined in JavaScript with WebAssembly's `wasm32-unknown-unknown` target

These examples have been built throughout time and were showcased in the following talks and meetups:
- **WeAreDevelopers World Congress**, "Type-safe bindings for Node.js with Rust and WebAssembly" (30 minutes version), Berlin (Germany), July 2023
- **Graz.js Meetup**, "Type-safe bindings for Node.js with Rust and WebAssembly" (20 minutes version), Graz (Austria), May 2023
- **Trivago's Rust Guild Meetup**, "WebAssembly in Production: From Rust to TypeScript and back", Düsseldorf (Germany), April 2023
- **Node Congress**, "Type-safe bindings for Node.js with Rust and WebAssembly" (20 minutes version), Berlin (Germany), April 2023
- **EuroRust**, "No free lunch: Limits of Wasm as a bridge from Rust to JS", Berlin (Germany), October 2022

I encourage you to ask me questions about these examples or topics related to WebAssembly, Rust, and TypeScript:
- on Twitter: [@jkomyno](https://twitter.com/jkomyno).
- in real life, if you happen to meet me at a conference, workshop, or a meetup 🤠. You can find a pic of myself below: 

<center>

![Alberto Schiabel](https://avatars.githubusercontent.com/u/12381818?s=400)

</center>

## Inspirations

This repository wouldn't have been possible without the following projects:

- [`js_sys`](https://github.com/rustwasm/wasm-bindgen/tree/main/crates/js-sys)
- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)
- [`wasm-bindgen-futures`](https://github.com/rustwasm/wasm-bindgen/tree/main/crates/futures)
- [`serde`](https://github.com/serde-rs/serde)
- [`serde_json`](https://github.com/serde-rs/json)
- [`serde-wasm-bindgen`](https://github.com/cloudflare/serde-wasm-bindgen)
- [`tsify`](https://github.com/madonoharu/tsify)

Please consider starring, supporting, and contributing to them.

## Get Started

### Requirements

- [`nodejs@18.16.1`](https://nodejs.org/en/download/) or superior*
- [`pnpm@7.20.0`](https://pnpm.io/installation) or superior*

(*) These are the versions used to develop this repository. Older versions might work as well, but they haven't been tested.

### Install Dependencies

- Install dependencies:
  ```sh
  pnpm i
  ```

In [`./rust`](./rust):

- Install the Rust toolchain via [Rustup](https://rustup.rs/):
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Add suppport for the `wasm32-unknown-unknown` compilation target for Rust:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```

- Install `wasm-bindgen`:
  ```sh
  cargo install -f wasm-bindgen-cli@0.2.87
  ```
  
  (the specific version is important, as `wasm-bindgen-cli` doesn't yet follow semantic versioning. This version needs to match the version of the `wasm-bindgen` dependency in the `Cargo.toml` files of the Rust crates)

### Build & Test

With Docker:

  - Build and run the local Docker image:

    ```sh
    ./build.sh
    ```

Without Docker:

  - Run Rust unit tests and build the WebAssembly artifacts:

    ```sh
    pnpm build:wasm
    ```

  - Run Node.js unit tests:

    ```sh
    pnpm test:ci
    ```

## Examples

### `read-function-from-object`

Can you pass an object of functions from JS to Rust, and call them from Rust?
It turns out you can, either by using `js_sys` or `wasm-bindgen(getter_with_clone)`.

Please refer to the [Rust code](./rust/read-function-from-object/src/lib.rs) and the [Node.js tests](./nodejs/demo/__tests__/read-function-from-object.test.ts) for more details.

## 👤 Author

**Alberto Schiabel**

* Twitter: [@jkomyno](https://twitter.com/jkomyno)
* Github: [@jkomyno](https://github.com/jkomyno)

Please consider supporting my work by following me on Twitter and starring my projects on GitHub.
I mostly post about TypeScript, Rust, and WebAssembly. Thanks!

## 📝 License

Built with ❤️ by [Alberto Schiabel](https://github.com/jkomyno).
This project is [MIT](https://github.com/jkomyno/rust-wasm-examples/blob/main/LICENSE) licensed.
