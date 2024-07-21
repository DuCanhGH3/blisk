<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { tokens }: RendererProps<Tokens.Blockquote> = $props();
</script>

<blockquote>
  <Parser {tokens} />
</blockquote>
