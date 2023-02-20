import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";


export default defineConfig({
  base: process.env.NODE_ENV === "production" ? "/mnemonic-pictures/" : "/",

  plugins: [
    wasm(),
    topLevelAwait(),
  ],

  optimizeDeps: {
    exclude: [
      "@gregorykogan/mnemonic-pictures"
    ]
  }
});
