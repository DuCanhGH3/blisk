<script lang="ts">
  import { enhance } from "$app/forms";
  import DatePicker from "$components/DatePicker.svelte";
  import Button from "$components/Button.svelte";
  import SelectNative from "$components/SelectNative.svelte";

  const { data, form } = $props();

  let isLoading = $state(false);
</script>

<div class="flex w-full items-center justify-center self-stretch p-4">
  <div class="container flex w-[90dvw] max-w-[500px] flex-col items-center gap-6 rounded-lg p-8 shadow-xl">
    <form
      method="POST"
      class="flex w-full flex-col gap-3"
      use:enhance={() => {
        isLoading = true;
        return async ({ update }) => {
          await update();
          isLoading = false;
        };
      }}
    >
      <h1 class="h2">Start reading</h1>
      <SelectNative name="book_name" id="read-book-name" label="Book" values={data.books.map(({ name, title }) => [name, title])} />
      <DatePicker
        name="starts_at"
        label="Starting date"
        id="read-book-starting"
        required
        value={data.now}
        min={data.now}
        max={data.oneYearLater}
        errorTextId="read-book-starting-error"
        errorText={form?.validationError?.starts_at}
      />
      <DatePicker
        name="ends_at"
        label="Ending date"
        id="read-book-ending"
        required
        value={data.now}
        min={data.now}
        max={data.oneYearLater}
        errorTextId="read-book-ending-error"
        errorText={form?.validationError?.ends_at}
      />
      <div class="flex w-full flex-row-reverse flex-wrap items-center gap-4">
        <Button as="button" type="submit" disabled={isLoading}>Start reading</Button>
        <Button as="a" variant="light" href="/books">Cancel</Button>
        {#if !!form?.error}
          {#if typeof form.error === "string"}
            <p class="text-error-light dark:text-error-dark" role="alert">{form.error}</p>
          {:else}
            <ul class="flex flex-row gap-2" role="alert">
              {#each form.error as error}
                <li class="text-error-light dark:text-error-dark">{error}</li>
              {/each}
            </ul>
          {/if}
        {/if}
      </div>
    </form>
  </div>
</div>
