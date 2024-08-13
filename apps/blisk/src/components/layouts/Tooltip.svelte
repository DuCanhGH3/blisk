<script lang="ts">
  import { tick } from "svelte";
  import { hotkeys } from "$lib/hotkeys.svelte";
  import { tooltip } from "$lib/stores/tooltip.svelte";
  import { fade } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { clsx } from "$lib/clsx";

  const id = $derived(tooltip.state.id);
  const text = $derived(tooltip.state.text);
  const x = $derived(tooltip.state.x ?? 0);
  const y = $derived(tooltip.state.y ?? 0);
  const right = $derived(tooltip.state.right);
  const bottom = $derived(tooltip.state.bottom);
  const maxHeight = $derived(tooltip.state.maxHeight);
  const closeTooltip = $derived(tooltip.state.closeTooltip);
  const timeout = $derived(tooltip.state.timeout);
  let width = $state(1);
  let tooltipContainer = $state<HTMLDivElement | null>(null);

  $effect(() => {
    if (text) {
      tick().then(() => {
        if (tooltipContainer) {
          width = tooltipContainer.getBoundingClientRect().width;
        }
      });
    }
  });

  hotkeys([["Escape", () => text && closeTooltip()]]);
</script>

{#if text}
  <div
    bind:this={tooltipContainer}
    {id}
    onmouseenter={() => clearTimeout(timeout)}
    onmouseleave={closeTooltip}
    role="tooltip"
    class={clsx(
      "absolute z-50 inline-flex select-none flex-col overflow-auto rounded-md border",
      "border-border-light bg-white text-left dark:border-border-dark dark:bg-neutral-915",
      "text-inherit shadow-[rgb(0_0_0/0.08)_0px_1px_4px] duration-100",
      "translate-x-[--offset] px-2 py-1.5 hover:select-auto"
    )}
    style="{right ? 'right' : 'left'}:{x}px;{bottom ? 'bottom' : 'top'}:{y}px"
    style:max-width="{window.innerWidth - x}px"
    style:max-height="{maxHeight}px"
    style:--offset="{Math.min(-10, window.innerWidth - (x + width + 10))}px"
    transition:fade={{ duration: 150, easing: quintOut }}
  >
    <span class="sr-only">Press Esc to dismiss this tooltip.</span>
    {text}
  </div>
{/if}
