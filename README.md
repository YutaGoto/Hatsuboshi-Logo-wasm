# Hatsuboshi Logo - Rust WebAssembly

[![clippy](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/clippy.yml/badge.svg)](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/clippy.yml)
[![reviewdog](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/reviewdog.yml/badge.svg)](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/reviewdog.yml)
[![rustfmt](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/rustfmt.yml)
[![security_audit](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/audit.yml/badge.svg)](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/audit.yml)
[![Dependabot Updates](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/YutaGoto/Hatsuboshi-Logo-wasm/actions/workflows/dependabot/dependabot-updates)


## How to install

```sh
pnpm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
pnpm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
pnpm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
pnpm test -- --firefox

# Runs tests in Chrome
pnpm test -- --chrome

# Runs tests in Safari
pnpm test -- --safari
```

## What does each file do?

* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

* `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

* The `tests` folder contains your Rust unit tests.
