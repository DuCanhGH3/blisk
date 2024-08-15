<script lang="ts">
  import Input from "$components/Input.svelte";
  import { fetchBackend } from "$lib/backend.client";
  import type { Book } from "$lib/types";
  import { debounce } from "$lib/utils";

  let searchResult = $state<Book[] | null>(null);

  const loadData = async (query: string) => {
    if (!query) {
      searchResult = [];
      return;
    }

    const data = await fetchBackend<Book[]>(`/books?q=${query}`);

    if (data.ok) {
      searchResult = data.data;
    }
  };

  const loadDataDebounced = debounce(loadData, 250);
</script>

<form action="/books" class="relative w-fit md:w-full">
  <Input
    id="search-bar-input"
    name="q"
    label="Search"
    autocapitalize="false"
    autocomplete="off"
    oninput={(e) => loadDataDebounced(e.currentTarget.value)}
  />
  {#if searchResult}
    <div class="absolute bottom-0 translate-y-full">
      {JSON.stringify(searchResult)}
    </div>
  {/if}
</form>
