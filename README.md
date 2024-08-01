## farm-js-plugin-pwa

A pwa rust plugin for farm

### Install

```bash
pnpm add -D farm-plugin-pwa
```

> [!NOTE]
> "@farmfe/core": "^1.2.0" --> "farm-plugin-pwa": "0.0.2"  
> "@farmfe/core": "^1.3.0" --> "farm-plugin-pwa": "^0.1.x"

### Usage

```ts
import { defineConfig } from "@farmfe/core";
import pwa from "farm-plugin-pwa";

interface Options {
  /* Your options here */
  /**
   * scope of sw cache
   */
  scope?: string; // default: publicPath of the farm config
  /**
   * name of the sw.js
   */
  sw_name?: string; // default: 'sw'
  /**
   * name of sw cache
   */
  cache_name?: string; // default: 'sw-cache'
  /**
   * custom cache files
   */
  static_files?: string[]; // ['/favicon.ico']
  /**
   * request url reg
   */
  patten?: string; // default:  /(.html|.js|.mjs|.css|.png|.jpg|.jpeg|.svg|.webp|.svga)$/
  /**
   * Web app manifests
   * see https://developer.mozilla.org/en-US/docs/Web/Manifest
   */
  manifest?: Record<string, any>;
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
    // or
    pwa({
      cache_name: `/front/pwa/`,
      sw_name: "pwa-sw",
      static_files: [
        "/front/pwa/favicon.ico",
        "/front/pwa/robots.txt",
        "/front/pwa/safari-pinned-tab.svg",
      ],
      manifest: {
        id: "Test_PWA",
        name: "Test_PWA",
        short_name: "Test_PWA",
        theme_color: "#ffffff",
        background_color: "#ffffff",
        start_url: "/front/pwa/",
        scope: "/front/pwa/",
        display: "standalone",
        display_override: ["fullscreen", "minimal-ui"],
        icons: [
          {
            src: "/front/pwa/pwa-192x192.png",
            sizes: "192x192",
            type: "image/png",
          },
          {
            src: "/front/pwa/pwa-512x512.png",
            sizes: "512x512",
            type: "image/png",
          },
          {
            src: "/front/pwa/pwa-512x512.png",
            sizes: "512x512",
            type: "image/png",
            purpose: "any maskable",
          },
        ],
      },
    }),
  ],
});
```
