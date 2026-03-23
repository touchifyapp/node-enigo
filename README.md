# `@touchifyapp/enigo`

![https://github.com/touchifyapp/node-enigo/actions](https://github.com/touchifyapp/node-enigo/workflows/CI/badge.svg)

> Cross platform input simulation. Node bindings for the [enigo](https://github.com/enigo-rs/enigo) crate.

Enigo lets you simulate mouse and keyboard input-events as if they were made by the actual hardware. It is available on Linux (X11), macOS and Windows.

It can be used for testing user interfaces on different platforms, building remote control applications or just automating tasks for user interfaces unaccessible by a public API or scripting language.

This library is in an early alpha status, the API will change in in the future.

## Compatibility

- [x] Serialize/Deserialize
- [x] Linux (X11) mouse
- [x] Linux (X11) text
- [x] Linux (Wayland) mouse (Experimental)
- [x] Linux (Wayland) text (Experimental)
- [ ] Linux (libei) mouse (Experimental)
- [ ] Linux (libei) text (Experimental)
- [x] MacOS mouse
- [x] MacOS text
- [x] Windows mouse
- [x] Windows text

## Installation

```bash
# npm
npm i @touchifyapp/enigo

# pnpm
pnpm add @touchifyapp/enigo

#yarn
yarn add @touchifyapp/enigo
```

## Usage

```typescript
import { Enigo, Button } from "@touchifyapp/enigo";

const enigo = Enigo.create();

// get mouse position
const [x, y] = enigo.getMousePosition();

// move mouse to position [100, 100];
enigo.mouseMove(100, 100);

//mouse operations
enigo.mouseDown("left");
enigo.mouseUp("left");
enigo.mouseClick("left");

//mouse operations
enigo.keyDown("alt");
enigo.keyUp("b");
enigo.keyPress("volumeup");
```

## Example

```typescript
import { Enigo, Button } from "@touchifyapp/enigo";

const enigo = Enigo.create();

// Paste
enigo.keyDown("control");
enigo.keyPress("v");
enigo.keyUp("control");

// Do things with the mouse
enigo.mouseMove(500, 200);
enigo.mouseDown(Button.Left);
enigo.mouseMove(600, 300);
enigo.mouseUp(Button.Left);

// Enter text
enigo.typeText("hello world");
```

## Contribute

### Prerequistes

- Install the latest `Rust`
- Install `Node.js@20+` which fully supported `Node-API`
- Run `corepack enable`

### Getting started

```bash
# clone repository
git clone https://github.com/touchifyapp/node-enigo
cd node-enigos

# install packages
pnpm i

# build and test
pnpm build
pnpm test
```

### Build

After `pnpm build` command, you can see `enigo.[darwin|win32|linux].node` file in project root.
This is the native addon built from [lib.rs](./src/lib.rs).

### Test

Run `pnpm test` to testing native addon.

> Note: a build is necessary before running `pnpm test` if changes were applied on the rust code.

### CI

With GitHub Actions, each commit and pull request will be built and tested automatically in [`node@20`, `node@22`, `node@24`] x [`macOS`, `Linux`, `Windows`] matrix.

## LICENSE

[MIT](./LICENSE)
