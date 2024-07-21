<script lang="ts">
  import type { RendererProps, Tokens } from "./types";

  const { href, title, text }: RendererProps<Tokens.Image> = $props();
</script>

<img src={href} {title} alt={text} />
