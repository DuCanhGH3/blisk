// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
import type { PrismaClient } from "@prisma/client";
import type { User } from "$lib/types";

declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      user: User | null;
      prisma: PrismaClient;
    }
    interface PageData {
      title?: string;
      ogImage?: string;
    }
    // interface PageState {}
    interface Platform {
      env?: {
        DATABASE: D1Database;
      };
      context: {
        waitUntil(promise: Promise<unknown>): void;
      };
      caches: CacheStorage & { default: Cache };
    }
  }
  interface ViewTransition {
    updateCallbackDone: Promise<void>;
    ready: Promise<void>;
    finished: Promise<void>;
    skipTransition: () => void;
  }

  interface Document {
    startViewTransition(updateCallback: () => Promise<void>): ViewTransition;
  }
}
