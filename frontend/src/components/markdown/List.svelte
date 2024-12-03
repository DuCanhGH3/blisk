<script lang="ts">
  import type { Tokens } from "marked";
  import ListItem from "./ListItem.svelte";
  import type { RendererProps } from "./types";

  const { ordered, items, start }: RendererProps<Tokens.List> = $props();
</script>

{#if ordered}
  <ol class="list-inside list-disc break-words pl-4 marker:text-wood-900 dark:marker:text-white" start={start || null}>
    {#each items as item}
      <ListItem {...item} />
    {/each}
  </ol>
{:else}
  <ul class="list-inside list-disc break-words pl-4 marker:text-wood-900 dark:marker:text-white">
    {#each items as item}
      <ListItem {...item} />
    {/each}
  </ul>
{/if}
