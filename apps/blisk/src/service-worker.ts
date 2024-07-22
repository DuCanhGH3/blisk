import { defaultCache } from "@serwist/vite/worker";
import { CacheFirst, ExpirationPlugin, type PrecacheEntry, RuntimeCaching, Serwist } from "serwist";

declare global {
  interface WorkerGlobalScope {
    __SW_MANIFEST: (PrecacheEntry | string)[] | undefined;
  }
}

declare const self: ServiceWorkerGlobalScope;

const runtimeCaching: RuntimeCaching[] = [
  {
    matcher({ url }) {
      return url.host === "wsrv.nl";
    },
    handler: new CacheFirst({
      cacheName: "wsrv-image-assets",
      plugins: [
        new ExpirationPlugin({
          maxEntries: 128,
          maxAgeSeconds: 365 * 24 * 60 * 60, // 365 days
          maxAgeFrom: "last-fetched",
        }),
      ],
    }),
  },
];

const serwist = new Serwist({
  precacheEntries: self.__SW_MANIFEST,
  precacheOptions: {
    cleanupOutdatedCaches: true,
    concurrency: 20,
    ignoreURLParametersMatching: [/^x-sveltekit-invalidated$/],
  },
  skipWaiting: true,
  clientsClaim: true,
  navigationPreload: false,
  disableDevLogs: true,
  runtimeCaching: [...(import.meta.env.DEV ? [] : runtimeCaching), ...defaultCache],
});

serwist.addEventListeners();
