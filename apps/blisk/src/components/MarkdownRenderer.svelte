<script lang="ts">
  import { setContext } from "svelte";
  import type { HeadingLevel } from "$lib/types";
  import { markdown } from "./markdown";
  import Parser from "./markdown/Parser.svelte";

  interface MarkdownRendererProps {
    source: string;
    startingHeading: HeadingLevel;
  }

  const { source, startingHeading }: MarkdownRendererProps = $props();

  setContext("startingHeading", startingHeading);

  const tokens = $derived(markdown.lexer(source));
</script>

<div class="[&>:first-child]:!mt-0 [&>:last-child]:!mb-0">
  <Parser {tokens} />
</div>
