<script lang="ts">
  import Bookshelf from "$components/Bookshelf.svelte";

  const { data } = $props();
</script>

{#if data.data.books.length > 0}
  <Bookshelf books={data.data.books} />
{:else}
  <div class="flex flex-col gap-4">
    <h3>No book read!</h3>
    <h4>Isn't it about time this user filled their bookshelf with books? Tell 'em!</h4>
  </div>
{/if}
