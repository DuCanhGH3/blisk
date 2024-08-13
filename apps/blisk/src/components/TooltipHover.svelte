<script lang="ts">
  // TODO: Allow customizing hover timeout to be compliant with Timing Adjustable.
  import { tooltip } from "$lib/stores/tooltip.svelte";
  import type { HTMLAttributes, MouseEventHandler } from "svelte/elements";

  interface TooltipHoverProps extends Omit<HTMLAttributes<HTMLSpanElement>, "role" | "aria-describedby"> {
    text: string;
    tooltipId: string;
  }

  const { text, tooltipId, children, onmouseenter, onmouseleave, ...props }: TooltipHoverProps = $props();
  let span = $state<HTMLSpanElement | null>(null);
  const isActive = $derived(tooltip.state.id === tooltipId);

  const mouseenter: MouseEventHandler<HTMLSpanElement> = (ev) => {
    clearTimeout(tooltip.state.timeout);

    if (span) {
      const rect = span.getBoundingClientRect();
      const viewportRect = document.getElementById("root-container")!.getBoundingClientRect();
      let x = (rect.left + rect.right) / 2 + window.scrollX;
      let right = false;
      let y = Math.max(0, viewportRect.bottom - rect.y);
      let bottom = true;

      if (window.innerWidth - x < 200) {
        x = Math.max(0, window.innerWidth - rect.right);
        right = true;
      }
      if (viewportRect.bottom - y < 200) {
        y = rect.top + window.scrollY + 24;
        bottom = false;
      }

      tooltip.state = {
        id: tooltipId,
        text,
        x,
        y,
        right,
        bottom,
        maxHeight: bottom ? rect.y : window.innerHeight - rect.y - 24,
        closeTooltip() {
          clearTimeout(tooltip.state.timeout);
          tooltip.reset();
        },
        timeout: undefined,
      };
    }

    onmouseenter?.(ev);
  };

  const mouseleave: MouseEventHandler<HTMLSpanElement> = (ev) => {
    tooltip.state.timeout = setTimeout(tooltip.state.closeTooltip, 500);
    onmouseleave?.(ev);
  };
</script>

<span
  bind:this={span}
  role="status"
  onmouseenter={mouseenter}
  onmouseleave={mouseleave}
  aria-describedby={isActive ? tooltip.state.id : undefined}
  {...props}
>
  {#if children}
    {@render children()}
  {/if}
</span>
