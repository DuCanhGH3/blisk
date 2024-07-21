<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  const { children, ...props }: HTMLAttributes<HTMLTableSectionElement> = $props();
</script>

<tbody {...props}>
  {#if children}
    {@render children()}
  {/if}
</tbody>
