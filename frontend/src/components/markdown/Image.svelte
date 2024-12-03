<script lang="ts">
  import type { RendererProps, Tokens } from "./types";

  const { href, title, text }: RendererProps<Tokens.Image> = $props();
</script>

<picture>
  <source media="(min-width: 512px)" srcset="https://wsrv.nl/?url={href}&h=256&dpr=2)" />
  <source srcset="https://wsrv.nl/?url={href}&h=256&dpr=1)" />
  <img
    src="https://wsrv.nl/?url={href}&h=128&dpr=4)"
    {title}
    alt={text}
    height={512}
    class="h-64 w-auto object-contain lg:h-[32rem] mx-auto"
    loading="lazy"
    decoding="async"
  />
</picture>
