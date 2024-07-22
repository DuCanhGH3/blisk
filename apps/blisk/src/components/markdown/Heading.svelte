<script lang="ts">
  import Paragraph from "./Paragraph.svelte";
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { raw, text, depth, tokens }: RendererProps<Tokens.Heading> = $props();
</script>

{#if depth === 1}
  <h2 class="mt-6 mb-4">
    <Parser {tokens} />
  </h2>
{:else if depth === 2}
  <h3 class="mt-6 mb-4">
    <Parser {tokens} />
  </h3>
{:else if depth === 3}
  <h4 class="mt-6 mb-4">
    <Parser {tokens} />
  </h4>
{:else if depth === 4}
  <h5 class="mt-6 mb-4">
    <Parser {tokens} />
  </h5>
{:else if depth === 5}
  <h6 class="mt-6 mb-4">
    <Parser {tokens} />
  </h6>
{:else}
  <Paragraph {raw} {text} {tokens} />
{/if}
