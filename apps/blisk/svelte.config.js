// @ts-check
import adapter from "@sveltejs/adapter-auto";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  // compilerOptions: {
  //   customElement: true,
  // },
  kit: {
    adapter: adapter(),
    alias: {
      $components: "./src/components",
      $images: "./src/images",
    },
    csp: {
      directives: {
        "frame-src": ["self"],
        "connect-src": ["self", "http://localhost:8080"],
        "font-src": ["self"],
        "img-src": ["self", "https://wsrv.nl"],
        "object-src": ["self"],
        "script-src": ["self", "wasm-unsafe-eval", "strict-dynamic", "sha256-DjP3mqXEHW08gJZjCdT8u4O2YkjsRGagw6vMJOyKiN4="],
        "style-src": ["self", "https:", "unsafe-inline"],
      },
    },
    inlineStyleThreshold: 2048,
    paths: {
      relative: false,
    },
    prerender: {
      concurrency: 20,
    },
    serviceWorker: {
      register: false,
    },
    csrf: {
      checkOrigin: false,
    },
  },
};

export default config;
