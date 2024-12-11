<script lang="ts">
  import Button from "$components/Button.svelte";
  import { clsx } from "$lib/clsx";
  import { dialog } from "$lib/stores/dialog.svelte";
  import type { HTMLDialogAttributes } from "svelte/elements";

  const { class: className, ...props }: HTMLDialogAttributes = $props();

  let dialogElement = $state<HTMLDialogElement | null>(null);

  let scroll = 0;

  $effect(() => {
    if (dialog.state) {
      scroll = document.documentElement.scrollTop;
      document.documentElement.style.overflowY = "hidden";
      dialogElement?.showModal();
    } else {
      dialogElement?.close();
      document.documentElement.style.overflowY = "";
      document.documentElement.scrollTo({ top: scroll, behavior: "instant" });
    }
  });
</script>

{#if dialog.state}
  <dialog
    bind:this={dialogElement}
    class={clsx("box md text-wood-900 flex w-[90dvw] max-w-[500px] flex-col gap-6 p-8! dark:text-white", className)}
    aria-labelledby="dialog-title"
    aria-describedby="dialog-desc"
    {...props}
  >
    <div>
      <h1 class="h4 mb-4" id="dialog-title">{dialog.state.title}</h1>
      <p class="h5" id="dialog-desc">{dialog.state.description}</p>
    </div>
    <div class="flex flex-row flex-wrap gap-4">
      {#if dialog.state.type === "action"}
        <Button as="button" variant={dialog.state.closeVariant} onclick={dialog.state.onClose} class="flex-1">
          {dialog.state.closeText}
        </Button>
        <Button as="button" variant="light" onclick={dialog.state.onCancel} class="flex-1">
          {dialog.state.cancelText}
        </Button>
      {:else}
        <Button as="button" variant="light" onclick={dialog.state.onClose}>
          {dialog.state.closeText}
        </Button>
      {/if}
    </div>
  </dialog>
{/if}
