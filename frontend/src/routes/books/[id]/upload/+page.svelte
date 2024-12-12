<script lang="ts">
  import { enhance } from "$app/forms";
  import Button from "$components/Button.svelte";
  import Input from "$components/Input.svelte";
  import SelectNative from "$components/SelectNative.svelte";
  import Textarea from "$components/Textarea.svelte";

  const { data, form } = $props();

  let isLoading = $state(false);
</script>

<div class="flex w-full flex-col gap-6">
  <h2>Upload a chapter</h2>
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
    <SelectNative name="volume" id="upload-chapter-volume" label="Volume" values={data.metadata.volumes.map(({ id, name }) => ["" + id, name])} />
    <SelectNative name="language" id="upload-chapter-language" label="Language" values={data.languages.map(({ code, name }) => [code, name])} />
    <Input name="name" id="create-post-name" label="Title" errorTextId="create-post-title-error" errorText={form?.validationError?.name} required />
    <Textarea
      name="content"
      id="upload-chapter-content"
      label="Content"
      rows={5}
      errorTextId="upload-chapter-content-error"
      errorText={form?.validationError?.content}
      required
    />
    <Button as="button" type="submit" disabled={isLoading}>Upload chapter</Button>
    {#if form?.error}
      <p class="text-error-light dark:text-error-dark" role="alert">{form.error}</p>
    {/if}
  </form>
</div>
