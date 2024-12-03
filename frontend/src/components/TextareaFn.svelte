<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { RequireFields } from "$lib/types";
  import type { HTMLButtonAttributes } from "svelte/elements";

  interface TextareaFnProps extends RequireFields<HTMLButtonAttributes, "children"> {
    label: string;
  }

  const { label, children, class: className, ...props }: TextareaFnProps = $props();
</script>

<button class={clsx("hover:bg-wood-300 dark:hover:bg-wood-800 rounded-md p-1 transition-colors duration-100", className)} type="button" {...props}>
  {@render children()}
  <span class="sr-only">{label}</span>
</button>
