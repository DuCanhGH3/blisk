<script lang="ts">
  import Input from "$components/Input.svelte";
  import { fetchBackend } from "$lib/backend.client";
  import type { Book } from "$lib/types";
  import { debounce } from "$lib/utils";

  let activeElement = $state<HTMLElement | null>(null);
  let input = $state<HTMLInputElement | null>(null);
  let searchResult = $state<Book[] | null>(null);

  const loadData = async (query: string) => {
    if (!query) {
      searchResult = null;
      return;
    }

    const data = await fetchBackend<Book[]>(`/books?q=${query}&include_reviews=false`);

    if (data.ok) {
      searchResult = data.data;
    } else {
      searchResult = null;
    }
  };

  const loadDataDebounced = debounce(loadData, 500);
</script>

<svelte:document bind:activeElement={activeElement} />

<form action="/books" class="relative w-fit md:w-full">
  <Input
    bind:self={input}
    id="search-bar-input"
    name="q"
    label="Search"
    autocapitalize="false"
    autocomplete="off"
    oninput={(e) => loadDataDebounced(e.currentTarget.value)}
  />
  {#if activeElement === input && searchResult}
    <div class="absolute bottom-0 max-h-[min(50dvh,25rem)] w-full translate-y-full overflow-y-auto">
      {JSON.stringify(searchResult)}
    </div>
  {/if}
</form>
