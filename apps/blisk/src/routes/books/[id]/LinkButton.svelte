<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { HTMLAnchorAttributes } from "svelte/elements";

  const { href, children, ...props }: Omit<HTMLAnchorAttributes, "class"> = $props();
</script>

<a
  {href}
  class={clsx(
    "select-none rounded-md px-2 transition-colors duration-100",
    "dark:hover:bg-accent-dark bg-neutral-250 hover:bg-accent-light hover:text-white dark:bg-neutral-800 dark:hover:text-black"
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</a>
