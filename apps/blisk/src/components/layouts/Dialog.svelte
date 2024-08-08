<script lang="ts">
  import Button from "$components/Button.svelte";
  import { clsx } from "$lib/clsx";
  import { dialog } from "$lib/stores/dialog.svelte";
  import type { HTMLDialogAttributes } from "svelte/elements";

  interface DialogProps extends HTMLDialogAttributes {
    self?: HTMLDialogElement | null;
  }

  let { self = $bindable(), class: className, ...props }: DialogProps = $props();

  $effect(() => {
    if (dialog.state !== null) {
      self?.showModal();
    } else {
      self?.close();
    }
  });
</script>

<dialog bind:this={self} class={clsx("box md flex w-[90dvw] max-w-[500px] flex-col gap-8 !p-8 text-black dark:text-white", className)} {...props}>
  {#if dialog.state}
    <div>
      <h1 class="h4 mb-4">{dialog.state.title}</h1>
      <h2 class="h5">{dialog.state.description}</h2>
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
  {/if}
</dialog>
