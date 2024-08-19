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
    "gap-2 break-words px-3 py-2 text-base font-medium md:text-sm",
    $page.url.pathname === href ? "dark:bg-wood-800 bg-wood-300" : "hover:bg-wood-200 dark:hover:bg-wood-750"
  )}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</a>
