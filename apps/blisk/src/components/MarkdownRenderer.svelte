<script lang="ts">
  import { lexer } from "marked";
  import Parser from "./markdown/Parser.svelte";

  interface MarkdownRendererProps {
    source: string;
  }

  const { source }: MarkdownRendererProps = $props();

  const tokens = $derived(lexer(source));
</script>

<Parser {tokens} />
