import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  base: process.env.NODE_ENV === "production" ? "/mnemonic-pictures/" : "/",

  plugins: [
    vue(),
    wasm(),
    topLevelAwait(),
  ],

  optimizeDeps: {
    exclude: [
      "@gregorykogan/mnemonic-pictures"
    ]
  }
})
