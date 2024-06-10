<script>
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";

  const { form } = $props();

  let isLoading = $state(false);
</script>

<div class="h-full w-full max-w-6xl p-2 md:py-8">
  <h1 class="mb-8">post</h1>
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
    <Input name="title" id="create-post-title" label="title" errorTextId="create-post-title-error" errorText={form?.validationError?.title} />
    <Input
      name="content"
      id="create-post-content"
      label="content"
      errorTextId="create-post-content-error"
      errorText={form?.validationError?.content}
    />
    <button class="button" type="submit" disabled={isLoading}>post</button>
  </form>
</div>
