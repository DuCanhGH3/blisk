<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { raw, depth, tokens }: RendererProps<Tokens.Heading> = $props();
</script>

{#if depth === 1}
  <h1>
    <Parser {tokens} />
  </h1>
{:else if depth === 2}
  <h2>
    <Parser {tokens} />
  </h2>
{:else if depth === 3}
  <h3>
    <Parser {tokens} />
  </h3>
{:else if depth === 4}
  <h4>
    <Parser {tokens} />
  </h4>
{:else if depth === 5}
  <h5>
    <Parser {tokens} />
  </h5>
{:else if depth === 6}
  <h6>
    <Parser {tokens} />
  </h6>
{:else}
  {raw}
{/if}
