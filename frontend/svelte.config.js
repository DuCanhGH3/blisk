// @ts-check
import "dotenv/config";
import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

const publicBackend = /** @type {`https://${string}.${string}` | undefined} */ (process.env.PUBLIC_BACKEND_URL);

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
        "connect-src": ["self", ...(publicBackend ? [publicBackend] : [])],
        "font-src": ["self"],
        "img-src": ["self", "blob:", "https://wsrv.nl",  ...(publicBackend ? [publicBackend] : [])],
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
