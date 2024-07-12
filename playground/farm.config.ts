import { defineConfig } from "@farmfe/core";

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
    ["@farmfe/plugin-react", { runtime: "automatic" }],
    [
      "farm-plugin-pwa",
      {
        cache_name: `/front/pwa/`,
        sw_name: "pwa-sw",
      },
    ],
  ],
});
