<script lang="ts">
  import { enhance } from "$app/forms";
  import { page } from "$app/stores";
  import Button from "$components/Button.svelte";
  import FilePicker from "$components/FilePicker.svelte";
  import Input from "$components/Input.svelte";
  import Select from "$components/Select.svelte";
  import SelectNative from "$components/SelectNative.svelte";
  import Textarea from "$components/Textarea.svelte";

  const { data, form } = $props();

  let imageFiles = $state<FileList | null | undefined>(undefined);
  let videoFiles = $state<FileList | null | undefined>(undefined);
  let isLoading = $state(false);

  const bookName = $derived($page.url.searchParams.get("book"));
</script>

<div class="max-w-(--breakpoint-md) flex w-full flex-col gap-6 rounded-lg p-8 shadow-xl">
  <h1>Post</h1>
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
    <Input
      name="title"
      id="create-post-title"
      label="Title"
      errorTextId="create-post-title-error"
      errorText={form?.validationError?.title}
      required
    />
    <!-- TODO: implement searching -->
    <SelectNative
      name="book"
      id="create-post-book"
      label="Book"
      values={data.books.map(({ name, title }) => [name, title])}
      initialValue={bookName && data.books.some(({ name }) => name === bookName) ? bookName : undefined}
    />
    <Select
      id="create-post-book-reaction"
      multiple={false}
      legends={[
        {
          name: "reaction",
          label: "How do you feel about the book?",
          options: [
            { id: "like", name: "Positive" },
            { id: "dislike", name: "Negative" },
          ],
        },
      ]}
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
              <img
                src={url}
                class="h-6 w-6 rounded-full object-cover"
                title={file.name}
                width="24"
                height="24"
                loading="lazy"
                decoding="async"
                alt={file.name}
              />
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
