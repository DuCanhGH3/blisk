<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { header, tokens }: RendererProps<Tokens.TableCell> = $props();
</script>

{#if header}
  <th>
    <Parser {tokens} />
  </th>
{:else}
  <td>
    <Parser {tokens} />
  </td>
{/if}
