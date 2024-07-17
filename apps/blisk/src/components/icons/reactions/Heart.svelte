<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { ReactionProps } from "../types";

  const { animatable = true, ...props }: ReactionProps = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
  <title>Love</title>
  <path fill="url(#heart-gradient)" d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0Z" />
  <path
    class={clsx(animatable && "animate-love")}
    fill="#fff"
    d="M10.473 4C8.275 4 8 5.824 8 5.824S7.726 4 5.528 4c-2.114 0-2.73 2.222-2.472 3.41C3.736 10.55 8 12.75 8 12.75s4.265-2.2 4.945-5.34c.257-1.188-.36-3.41-2.472-3.41Z"
  />
</svg>
