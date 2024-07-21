<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { tokens }: RendererProps<Tokens.Strong> = $props();
</script>

<strong>
  <Parser {tokens} />
</strong>
