import { defineConfig } from "vite";
import suidPlugin from "@suid/vite-plugin";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  server: {
    host: "0.0.0.0",
  },
  plugins: [suidPlugin(), solidPlugin()],
  build: {
    target: "esnext",
  },
});
