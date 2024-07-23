<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { header, tokens }: RendererProps<Tokens.TableCell> = $props();
</script>

{#if header}
  <th class="dark:bg-neutral-915 bg-white transition-colors duration-100 px-2 py-1">
    <Parser {tokens} />
  </th>
{:else}
  <td class="dark:bg-neutral-915 bg-white transition-colors duration-100 px-2 py-1">
    <Parser {tokens} />
  </td>
{/if}
