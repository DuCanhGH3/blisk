<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { HTMLAttributes } from "svelte/elements";

  interface MenuProps extends Omit<HTMLAttributes<HTMLDivElement>, "role" | "tabindex"> {}

  const { class: className, children, ...props }: MenuProps = $props();
</script>

<div
  class={clsx(
    "bg-wood-hor dark:bg-dark-wood-hor absolute mt-2 rounded-[10px] shadow-lg transition ease-in-out [&>*]:p-1",
    "border-border-light dark:border-border-dark dark:bg-wood-800 bg-wood-300 border-[0.25px]",
    "divide-border-light dark:divide-border-dark divide-y ring-1 ring-black/5 focus:outline-none",
    className
  )}
  role="menu"
  tabindex={0}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</div>
