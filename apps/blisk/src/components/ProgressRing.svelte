<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { SVGAttributes } from "svelte/elements";

  const MIN_XY_COEFF = 2;
  const MIN_SIZE_ADD = 8;

  interface ProgressRingProps extends SVGAttributes<SVGElement> {
    size?: "lg" | "md" | "sm";
    determinate?: boolean;
    value?: number;
  }

  const { size: sizeLetter = "md", determinate = false, value = 0, class: className, style, ...props }: ProgressRingProps = $props();

  const length = $derived.by(() => {
    switch (sizeLetter) {
      case "lg":
        return 200;
      case "md":
        return 100;
      case "sm":
        return 75;
    }
  });

  const size = $derived.by(() => {
    switch (sizeLetter) {
      case "lg":
        return 64;
      case "md":
        return 32;
      case "sm":
        return 24;
    }
  });
</script>

<svg
  class={clsx(!determinate ? "animate-ring" : "-rotate-90", className)}
  style="width:{size}px;height:{size}px;{style}"
  viewBox="{size / MIN_XY_COEFF} {size / MIN_XY_COEFF} {size + MIN_SIZE_ADD} {size + MIN_SIZE_ADD}"
  width={size}
  height={size}
  {...props}
>
  <circle
    class={clsx(
      determinate ? "" : sizeLetter === "lg" ? "animate-ring-lg" : sizeLetter === "md" ? "animate-ring-md" : "animate-ring-sm",
      "stroke-accent-light dark:stroke-accent-dark"
    )}
    cx={size + MIN_SIZE_ADD / MIN_XY_COEFF}
    cy={size + MIN_SIZE_ADD / MIN_XY_COEFF}
    r={size / MIN_XY_COEFF}
    fill="none"
    stroke-linecap="round"
    stroke-width={sizeLetter === "lg" ? 8 : 4}
    stroke-miterlimit="10"
    stroke-dasharray={determinate ? length : undefined}
    stroke-dashoffset={determinate ? length - length * value * 0.01 : undefined}
  />
</svg>
