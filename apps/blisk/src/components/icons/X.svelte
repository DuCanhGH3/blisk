<script lang="ts">
  import type { IconProps } from "./types";

  const { ...props }: IconProps = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 24 24"
  stroke-width={2}
  stroke="currentColor"
  fill="none"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
  {...props}
>
  <path d="M18 6l-12 12" />
  <path d="M6 6l12 12" />
</svg>
