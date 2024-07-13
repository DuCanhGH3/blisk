<script lang="ts">
  import type { SVGAttributes } from "svelte/elements";

  const { children, ...props }: SVGAttributes<SVGElement> = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" {...props}>
  {#if children}
    {@render children()}
  {/if}
</svg>
