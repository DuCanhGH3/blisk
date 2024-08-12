<script lang="ts">
  import { enhance } from "$app/forms";
  import Button from "$components/Button.svelte";
  import FilePicker from "$components/FilePicker.svelte";
  import Input from "$components/Input.svelte";
  import Textarea from "$components/Textarea.svelte";

  const { form } = $props();

  let imageFiles = $state<FileList | null | undefined>(undefined);

  let videoFiles = $state<FileList | null | undefined>(undefined);

  let isLoading = $state(false);
</script>

<div class="flex w-full items-center justify-center self-stretch p-4">
  <div class="container flex w-[90dvw] max-w-[600px] items-center gap-6 rounded-lg p-8 shadow-xl">
    <h1 class="sr-only">Post</h1>
    <form
      method="POST"
      class="flex w-full flex-col gap-3"
      enctype="multipart/form-data"
      use:enhance={() => {
        isLoading = true;
        return async ({ update }) => {
          await update();
          isLoading = false;
        };
      }}
    >
      <div class="flex flex-1 flex-row items-center justify-stretch gap-2">
        <img
          src="/no-avatar.webp"
          class="border-border-light dark:border-border-dark h-auto w-[50px] select-none rounded-full border shadow-lg"
          width={50}
          height={50}
          alt=""
        />
        <Input
          name="title"
          id="create-post-title"
          label="Title"
          errorTextId="create-post-title-error"
          errorText={form?.validationError?.title}
          required
        />
      </div>
      <Input
        type="number"
        name="bookId"
        id="create-post-book-id"
        label="Book ID"
        errorTextId="create-post-book-id-error"
        errorText={form?.validationError?.book_id}
        required
      />
      <Textarea
        name="content"
        id="create-post-content"
        label="Content"
        rows={5}
        errorTextId="create-post-content-error"
        errorText={form?.validationError?.content}
        required
      />
      <div class="flex flex-row gap-2">
        <FilePicker
          bind:files={imageFiles}
          id="create-post-photo-picker"
          fileType="image"
          multiple
          name="images"
          errorTextId="create-post-photo-picker-error"
          errorText={form?.validationError?.images}
        />
        <FilePicker bind:files={videoFiles} id="create-post-video-picker" fileType="video" multiple name="videos" />
      </div>
      {#if imageFiles || videoFiles}
        <ol class="flex flex-row flex-wrap gap-2">
          {#if imageFiles}
            {#each imageFiles as file}
              {@const url = URL.createObjectURL(file)}
              <li class="border-border-light dark:border-border-dark dark:bg-neutral-915 flex flex-row gap-2 rounded-full border bg-white p-1">
                <img class="h-6 w-6 rounded-full object-cover" src={url} alt={file.name} title={file.name} width="24" height="24" />
                <p class="line-clamp-1 max-w-32 pr-1">{file.name}</p>
              </li>
            {/each}
          {/if}
        </ol>
      {/if}
      <Button as="button" type="submit" disabled={isLoading}>Post</Button>
      {#if form?.error}
        <p class="text-error-light dark:text-error-dark" role="alert">{form.error}</p>
      {/if}
    </form>
  </div>
</div>
