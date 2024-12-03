<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { HTMLAnchorAttributes } from "svelte/elements";

  const { href, children, ...props }: Omit<HTMLAnchorAttributes, "class"> = $props();
</script>

<a
  {href}
  class={clsx(
    "select-none rounded-md px-2 transition-colors duration-100",
    "bg-wood-300 hover:bg-wood-950 hover:text-wood-200 dark:bg-wood-800 dark:hover:bg-wood-500 dark:hover:text-wood-900"
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</a>
