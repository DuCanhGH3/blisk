<script lang="ts">
  import type { Token } from "marked";
  import { renderers } from "./index.js";
  import type { RendererKey } from "./types.js";

  const { tokens }: { tokens: Token[] } = $props();
</script>

{#each tokens as token}
  {#if token.type in renderers}
    {@const Component = renderers[token.type as RendererKey]}
    <Component {...token as any} />
  {:else}
    {token.raw}
  {/if}
{/each}
