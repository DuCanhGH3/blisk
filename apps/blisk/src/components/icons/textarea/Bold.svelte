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
  <path d="M7 5h6a3.5 3.5 0 0 1 0 7h-6z" />
  <path d="M13 12h1a3.5 3.5 0 0 1 0 7h-7v-7" />
</svg>
