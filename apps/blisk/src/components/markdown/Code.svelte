<script lang="ts">
  import { readonlyArrayIncludes } from "$lib/utils";
  import { highlighter, permittedLanguages } from "./shiki";
  import type { RendererProps, Tokens } from "./types";

  const { lang, text }: RendererProps<Tokens.Code> = $props();
</script>

<div class="dark:bg-neutral-915 mb-4 flex w-full flex-col rounded-xl border border-neutral-300 bg-white shadow-md dark:border-neutral-800">
  <div class="margin-0 overflow-auto p-4">
    {#if readonlyArrayIncludes(permittedLanguages, lang)}
      {@const highlighted = highlighter.codeToHtml(text, {
        lang,
        themes: {
          light: "github-light",
          dark: "github-dark",
        },
      })}
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html highlighted}
    {:else}
      <pre><code>{text}</code></pre>
    {/if}
  </div>
</div>
