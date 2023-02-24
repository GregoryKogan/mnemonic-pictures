# mnemonic-pictures
NPM package for generating mnemonic pictures written in Rust

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white&color=CE412B)
![image](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=white)
![image](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)  
[![NPM](https://nodei.co/npm/@gregorykogan/mnemonic-pictures.png?compact=true)](https://nodei.co/npm/@gregorykogan/mnemonic-pictures/)

**Check out the [online demo](https://gregorykogan.github.io/mnemonic-pictures)**

Generate a unique picture for any given (number/string) seed
<p align="center">
  <img alt="examples" src="https://raw.githubusercontent.com/GregoryKogan/GregoryKogan/main/mnemonic2.gif">
</p>

**Generation is presistent across devices and sessions**\
To verify this, you can enter 'Red Sky' seed at 195x130 resolution in the [online demo](https://gregorykogan.github.io/mnemonic-pictures)\
Resulting image should look like red sky
<p align="center">
  <img width="838" alt="image" src="https://user-images.githubusercontent.com/60318411/220775652-f1eb74d9-3e7a-45a4-a4e9-23978eda837a.png">
</p>

## Installation

NPM package: https://www.npmjs.com/package/@gregorykogan/mnemonic-pictures

**npm**
```shell
npm i @gregorykogan/mnemonic-pictures
```

**yarn**
```shell
yarn add @gregorykogan/mnemonic-pictures
```

## Setup

This package uses [WASM](https://webassembly.org/) which is not supported by default by most frontend tooling. 
You would need some 3rd party package to add WASM support. For example this is what you would need to do for [Vite](https://vitejs.dev/):

Add [vite-plugin-wasm](https://www.npmjs.com/package/vite-plugin-wasm) and configure it like this
```typescript
// vite.config.ts

import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
  ],

  optimizeDeps: {
    exclude: [
      "@gregorykogan/mnemonic-pictures"
    ]
  }
})
```

## Usage

### Import

WASM module needs to be initialized before running. `init()` function is async and is module's default export

Import example **with** top-level await:
```typescript
import init, { init_console_errors } from '@gregorykogan/mnemonic-pictures';

await init();
init_console_errors();

// do whatever with the module
```

Import example **without** top-level await:
```typescript
import init, { init_console_errors } from '@gregorykogan/mnemonic-pictures';

let wasmLoaded = false;

init().then(() => {
  init_console_errors();
  wasmLoaded = true;
});

// ...

if (wasmLoaded) {
  // do whatever with the module
}
```

Here `init_console_errors()` is optional. It will log human readable traceback for wasm errors if they occur.

### Generation

HTML
```html
<canvas id="display" width="600" height="400"></canvas>
```

Script
```typescript
import { generate, generate_from_string } from '@gregorykogan/mnemonic-pictures';

// assuming wasm module is initialized
generate("display", BigInt(123456));  // generate from number seed
// or
generate_from_string("display", "my-seed");  // generate from string seed
```
Here the first argument is an `id` of the canvas html element and the second one is a generation seed.
Seed is used to configure starting state of pseudo random generator that is used for further image generation on all steps.
To get u64 number from string [SipHasher crate](https://crates.io/crates/siphasher) is used.

### Blurry canvas issue
Often canvases of small sizes (15 by 10 for example) are displayed blurry. To fix it add this line to your canvas CSS styling:
```CSS
#display {
  image-rendering: pixelated;
}
```

### Example

You can see full usage example [here](https://github.com/GregoryKogan/mnemonic-pictures/tree/main/example)
