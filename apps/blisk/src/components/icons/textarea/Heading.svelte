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
  <path d="M7 12h10" />
  <path d="M7 5v14" />
  <path d="M17 5v14" />
  <path d="M15 19h4" />
  <path d="M15 5h4" />
  <path d="M5 19h4" />
  <path d="M5 5h4" />
</svg>
