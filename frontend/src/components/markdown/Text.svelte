<script lang="ts">
  import Parser from "./Parser.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { ...props }: RendererProps<Tokens.Text | Tokens.Escape | Tokens.Tag> = $props();
</script>

{#if "tokens" in props && props.tokens}
  <Parser tokens={props.tokens} />
{:else}
  {props.text}
{/if}
