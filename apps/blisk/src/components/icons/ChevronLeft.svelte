<script lang="ts">
  import type { SVGAttributes } from "svelte/elements";

  const { ...props }: SVGAttributes<SVGElement> = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width={2}
  stroke-linecap="round"
  stroke-linejoin="round"
  {...props}
>
  <path d="M15 6l-6 6l6 6"></path>
</svg>
