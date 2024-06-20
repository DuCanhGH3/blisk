<script>
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";
  import Textarea from "$components/Textarea.svelte";

  const { form } = $props();

  let isLoading = $state(false);
</script>

<div class="h-full w-full max-w-6xl p-2 md:py-8">
  <h1 class="mb-8">Post</h1>
  <form
    method="POST"
    class="flex flex-col gap-3"
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
    <button class="button" type="submit" disabled={isLoading}>Post</button>
    {#if form?.error}
      <p class="text-error">{form.error}</p>
    {/if}
  </form>
</div>
