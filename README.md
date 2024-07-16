## farm-js-plugin-pwa

A pwa rust plugin for farm

### Install

```bash
pnpm add -D farm-plugin-pwa
```

### Usage

```ts
import { defineConfig } from "@farmfe/core";

interface Options {
  /* Your options here */
  /**
   * scope of sw cache
   */
  scope: string; // default: publicPath of the farm config
  /**
   * name of the sw.js
   */
  sw_name: string; // default: 'sw'
  /**
   * name of sw cache
   */
  cache_name: string; // default: 'sw-cache'
  /**
   * custom cache files
   */
  static_files: string[]; // ['/favicon.ico']
  /**
   * request url reg
   */
  patten: string; // default:  /(.html|.js|.mjs|.css|.png|.jpg|.jpeg|.svg|.webp|.svga)$/
}

export default defineConfig({
  plugins: [
    [
      "farm-plugin-pwa",
      {
        cache_name: `/front/pwa/`,
        sw_name: "pwa-sw",
      },
    ],
  ],
});
```
