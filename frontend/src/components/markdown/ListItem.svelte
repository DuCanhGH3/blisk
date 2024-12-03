<script lang="ts">
  import type { Tokens } from "marked";
  import type { RendererProps } from "./types";
  import Parser from "./Parser.svelte";

  const { tokens }: RendererProps<Tokens.ListItem> = $props();
</script>

<li class="mb-2 [&>ol]:mt-2 [&>ul]:mt-2">
  <Parser {tokens} />
</li>
