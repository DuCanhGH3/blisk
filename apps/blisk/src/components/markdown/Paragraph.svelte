<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { tokens }: RendererProps<Tokens.Paragraph> = $props();
</script>

<p class="pb-4">
  <Parser {tokens} />
</p>
