import { defineConfig } from "@farmfe/core";
import pwa from "farm-plugin-pwa";
import react from "@farmfe/plugin-react";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    output: {
      path: `pwa`,
      publicPath: `/front/pwa/`,
      filename: "assets/[ext]/[name].[hash].[ext]",
      assetsFilename: "static/[resourceName].[ext]",
    },
    persistentCache: false,
    progress: false,
    runtime: {
      isolate: true,
    },
  },
  plugins: [
    react({ runtime: "automatic" }),
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
