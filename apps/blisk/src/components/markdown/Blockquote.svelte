<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { tokens }: RendererProps<Tokens.Blockquote> = $props();
</script>

<blockquote class="text-comment border-neutral-600 dark:border-neutral-400 border-l-2 px-4">
  <Parser {tokens} />
</blockquote>
