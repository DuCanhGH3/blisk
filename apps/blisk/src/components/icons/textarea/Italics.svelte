<script lang="ts">
  import type { IconProps } from "../types";

  const { ...props }: IconProps = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  {...props}
>
  <path d="M11 5l6 0" />
  <path d="M7 19l6 0" />
  <path d="M14 5l-4 14" />
</svg>
