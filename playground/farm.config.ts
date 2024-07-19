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
    }),
  ],
});
