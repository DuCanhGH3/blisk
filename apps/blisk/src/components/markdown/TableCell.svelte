<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { header, tokens }: RendererProps<Tokens.TableCell> = $props();
</script>

{#if header}
  <th class="dark:bg-neutral-915 border-border-light dark:border-border-dark border bg-white px-2 py-1 transition-colors duration-100">
    <Parser {tokens} />
  </th>
{:else}
  <td class="dark:bg-neutral-915 border-border-light dark:border-border-dark border bg-white px-2 py-1 transition-colors duration-100">
    <Parser {tokens} />
  </td>
{/if}
