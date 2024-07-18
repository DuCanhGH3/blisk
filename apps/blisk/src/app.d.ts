// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
import type { User } from "$lib/types";
import { EmblaCarouselType } from "embla-carousel";

declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      user: User | null;
    }
    interface PageData {
      title?: string;
      ogImage?: string;
    }
    // interface PageState {}
    // interface Platform {}
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

  declare namespace svelteHTML {
    interface HTMLAttributes<T> {
      onemblaInit?: (evt: CustomEvent<EmblaCarouselType>) => void;
    }
  }
}
