<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { href, title, tokens }: RendererProps<Tokens.Link> = $props();
</script>

{#if href.startsWith("/")}
  <a {href} {title} class="link">
    <Parser {tokens} />
  </a>
{:else}
  <a {href} {title} class="link" target="_blank" rel="noopener noreferrer">
    <Parser {tokens} />
  </a>
{/if}
