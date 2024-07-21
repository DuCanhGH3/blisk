<script lang="ts">
  import type { Token } from "marked";
  import { renderers } from "./index.js";
  import type { RendererKey } from "./types.js";

  const { tokens }: { tokens: Token[] } = $props();
</script>

{#each tokens as token}
  {#if token.type in renderers}
    <svelte:component this={renderers[token.type as RendererKey]} {...token as any} />
  {:else}
    {token.raw}
  {/if}
{/each}
