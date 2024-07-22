<script lang="ts">
  import { markdown } from "./markdown/index.js";
  import Parser from "./markdown/Parser.svelte";

  interface MarkdownRendererProps {
    source: string;
  }

  const { source }: MarkdownRendererProps = $props();

  const tokens = $derived(markdown.lexer(source));
</script>

<Parser {tokens} />
