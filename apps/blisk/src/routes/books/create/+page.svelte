<script>
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";

  const { form } = $props();

  let isLoading = $state(false);
</script>

<div class="h-full w-full max-w-6xl p-2 md:py-8">
  <h1 class="mb-8">Book</h1>
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
    <Input name="title" id="create-book-title" label="Title" errorTextId="create-book-title-error" errorText={form?.validationError?.title} />
    <Input
      name="summary"
      id="create-book-content"
      label="Content"
      errorTextId="create-book-content-error"
      errorText={form?.validationError?.summary}
    />
    <button class="button" type="submit" disabled={isLoading}>Create</button>
    {#if form?.error}
      <p class="text-error">{form.error}</p>
    {/if}
  </form>
</div>
