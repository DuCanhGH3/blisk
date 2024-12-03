<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAnchorAttributes } from "svelte/elements";

  import { clsx } from "$lib/clsx";

  interface NavLinkProps extends HTMLAnchorAttributes {
    href: string;
    isActive?: boolean;
    wideText?: boolean;
    textCenter?: boolean;
    button?: Snippet<[]>;
  }

  const { href, isActive = false, wideText = false, textCenter = true, children, button, ...props }: NavLinkProps = $props();
</script>

<span
  class={clsx(
    "transition-colors-opacity flex w-full cursor-pointer select-none flex-row justify-between rounded-md duration-100",
    isActive ? "dark:bg-wood-800 bg-wood-300" : "hover:bg-wood-200 dark:hover:bg-wood-750"
  )}
>
  <a
    {href}
    class={clsx(
      "h-full w-full gap-2 break-words px-3 py-2 font-medium",
      textCenter && "text-center",
      wideText ? "shrink-0 text-base uppercase tracking-widest" : "text-base md:text-sm"
    )}
    aria-current={isActive ? "page" : undefined}
    {...props}
  >
    {#if children}
      {@render children()}
    {/if}
  </a>
  {#if button}
    {@render button()}
  {/if}
</span>
