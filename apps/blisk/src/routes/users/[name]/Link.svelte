<script lang="ts">
  import { page } from "$app/stores";
  import { clsx } from "$lib/clsx";
  import type { HTMLAnchorAttributes } from "svelte/elements";

  interface LinkProps extends Omit<HTMLAnchorAttributes, "class"> {
    href: `/${string}`;
  }

  const { href, children, ...props }: LinkProps = $props();
</script>

<a
  {href}
  class={clsx(
    "transition-colors-opacity flex cursor-pointer select-none items-center justify-center rounded-md duration-100",
    "gap-2 break-words px-3 py-2 text-base font-medium text-wood-950 md:text-sm dark:text-white",
    $page.url.pathname === href ? "bg-neutral-250 dark:bg-neutral-800" : "hover:bg-neutral-250 dark:hover:bg-neutral-800"
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</a>
