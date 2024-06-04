<script>
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";

  const { form } = $props();

  let isUploading = $state(false);
</script>

<div class="flex w-full flex-row items-center justify-center gap-4 self-stretch p-4">
  <div class="container flex w-[90dvw] max-w-[500px] items-center gap-6 rounded-lg p-8 shadow-xl transition-all">
    <form
      method="POST"
      enctype="multipart/form-data"
      class="flex w-full flex-col gap-3"
      use:enhance={() => {
        isUploading = true;
        return async ({ update }) => {
          isUploading = false;
          await update();
        };
      }}
    >
      <h1>Upload a PDF</h1>
      <Input
        name="title"
        type="text"
        label="Title"
        id="upload-pdf-title-input"
        required
        errorText={form?.titleError}
        errorTextId="upload-pdf-title-error"
      />
      <input name="file" type="file" id="upload-pdf-file-input" required />
      {#if !!form?.fileError}
        <p class="text-error" id="upload-pdf-file-error">{form.fileError}</p>
      {/if}
      <button class="button filled" disabled={isUploading} type="submit">Upload</button>
    </form>
  </div>
</div>

{#if form?.data}
  <div>
    {#each form.data as str}
      <pre>{str}</pre>
    {/each}
  </div>
{/if}
