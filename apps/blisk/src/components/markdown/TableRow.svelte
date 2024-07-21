<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  const { children, ...props }: HTMLAttributes<HTMLTableRowElement> = $props();
</script>

<tr {...props}>
  {#if children}
    {@render children()}
  {/if}
</tr>
