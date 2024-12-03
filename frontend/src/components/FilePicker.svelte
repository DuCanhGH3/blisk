<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import PhotoFilled from "./icons/PhotoFilled.svelte";
  import VideoFilled from "./icons/VideoFilled.svelte";
  import { clsx } from "$lib/clsx";

  type FileType = "image" | "video";

  interface FilePickerProps extends Omit<HTMLInputAttributes, "accept" | "aria-invalid" | "aria-describedby" | "class" | "type"> {
    id: string;
    errorTextId?: string;
    errorText?: string | string[];
    files?: FileList | null | undefined;
    fileType: FileType;
  }

  let { id, errorTextId, errorText, files = $bindable(), fileType, multiple, ...props }: FilePickerProps = $props();

  const accept = $derived.by(() => {
    switch (fileType) {
      case "image":
        return ".apng, .bmp, .gif, .jpeg, .pjpeg, .png, .svg+xml, .tiff, .webp, .x-icon";
      case "video":
        return ".mp4, .webm";
    }
  });
</script>

<label
  class={clsx(
    "flex cursor-pointer items-center justify-center rounded-full border bg-transparent p-2 shadow-md duration-100",
    "dark:bg-neutral-915 border-border-light dark:border-border-dark hover:bg-neutral-250 bg-white dark:hover:bg-neutral-800"
  )}
  for={id}
>
  {#if fileType === "image"}
    <PhotoFilled width={24} height={24} class="h-auto w-6" aria-hidden="true" />
    <span class="sr-only">
      {#if multiple}
        Pick photos
      {:else}
        Pick a photo
      {/if}
    </span>
  {:else if fileType === "video"}
    <VideoFilled width={24} height={24} class="h-auto w-6" aria-hidden="true" />
    <span class="sr-only">
      {#if multiple}
        Pick videos
      {:else}
        Pick a video
      {/if}
    </span>
  {/if}
</label>
<input bind:files {id} class="sr-only" type="file" {accept} {multiple} aria-invalid={!!errorText} aria-describedby={errorTextId} {...props} />
{#if !!errorText && errorTextId}
  {#if typeof errorText === "string"}
    <p class="text-error-light dark:text-error-dark" id={errorTextId}>{errorText}</p>
  {:else}
    <div id={errorTextId} class="flex flex-col gap-2">
      {#each errorText as error}
        <p class="text-error-light dark:text-error-dark">{error}</p>
      {/each}
    </div>
  {/if}
{/if}
