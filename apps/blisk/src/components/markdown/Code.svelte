<script lang="ts">
  import type { RendererProps, Tokens } from "./types";

  const { lang, text }: RendererProps<Tokens.Code> = $props();
</script>

<div class="dark:bg-neutral-915 my-3 flex w-full flex-col rounded-xl border border-neutral-300 bg-white shadow-md dark:border-neutral-800">
  <div class="margin-0 overflow-auto p-4">
    <pre class={lang}><code>{text}</code></pre>
  </div>
</div>
