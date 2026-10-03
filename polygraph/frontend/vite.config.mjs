// SPDX-License-Identifier: MPL-2.0

import { defineConfig } from "vite";

export default defineConfig({
  server: {
    host: "0.0.0.0",
    port: 5173,
    strictPort: true,
    allowedHosts: [".e2b.app", "localhost", "127.0.0.1"],
    proxy: {
      "/graphql": "http://localhost:8000",
    },
  },
  preview: {
    host: "0.0.0.0",
    port: 4173,
    strictPort: true,
    allowedHosts: [".e2b.app", "localhost", "127.0.0.1"],
  },
});
