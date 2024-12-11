<script lang="ts">
  // TODO(ducanhgh): Work-in-progress.
  // import { goto, pushState } from "$app/navigation";
  // import { page } from "$app/stores";
  // import X from "$components/icons/X.svelte";
  // import PostRenderer from "$components/PostRenderer.svelte";
  // import { rendererButtonAttributes } from "$components/renderer-constants";
  // import VirtualScroller from "$components/VirtualScroller.svelte";
  // import Button from "$components/Button.svelte";
  import Bookshelf from "$components/Bookshelf.svelte";
  // import type { Book, MouseEventEvent, Post, Ref } from "$lib/types.js";

  const { data } = $props();

  // const selectedBook = $derived.by(() => {
  //   const state = $state("selected" in $page.state ? ($page.state.selected as Book) : undefined);
  //   return state;
  // });

  // const showModal = async (e: MouseEventEvent<HTMLAnchorElement>, bookName: string) => {
  //   if (
  //     window.innerWidth < 640 || // bail if the screen is too small
  //     e.shiftKey || // or the link is opened in a new window
  //     e.metaKey ||
  //     e.ctrlKey // or a new tab (mac: metaKey, win/linux: ctrlKey)
  //     // should also consider clicking with a mouse scroll wheel
  //   ) {
  //     return;
  //   }

  //   // prevent navigation
  //   e.preventDefault();

  //   const { href } = e.currentTarget;

  //   const selected = data.books.find((book) => book.name === bookName);

  //   if (selected) {
  //     pushState(href, { selected });
  //   } else {
  //     goto(href);
  //   }
  // };
</script>

{#if data.books.length > 0}
  <Bookshelf books={data.books} />
{:else}
  <div class="flex flex-col gap-6 px-8 py-8 md:px-0">
    <h1 class="h2">No books found!</h1>
    <h2 class="h3">It's time to fill the bookshelf, isn't it?</h2>
  </div>
{/if}
<!-- {#if selectedBook}
  <div class="fixed left-0 top-0 z-50 flex h-full w-full items-center justify-center bg-black/80">
    <div class="box lg relative flex w-[90dvw] max-w-(--breakpoint-lg) flex-col gap-4 p-8! text-wood-900 lg:flex-row lg:gap-8 dark:text-white">
      <Button as="button" variant="light" class="absolute right-4 top-4 z-2 p-2!" onclick={() => history.back()}>
        <X {...rendererButtonAttributes} />
        <span class="sr-only">Go back</span>
      </Button>
      <img src="/test-cover.jpg" class="h-72 w-48 rounded-lg" width="192" height="288" alt="" />
      <div class="z-1 flex flex-1 flex-col gap-2 break-all">
        <p class="h1 text-5xl lg:text-8xl">{selectedBook.title}</p>
        <p class="sr-only">About this book</p>
        <p class="text-comment text-3xl font-semibold leading-10 tracking-tight" aria-label="This book was written by Frank Herbert in 1965">
          {selectedBook.authors.map((author) => author.name).join(", ")} • 1965
        </p>
        <p class="max-h-72 overflow-y-auto lg:max-h-full">
          {selectedBook.summary}
        </p>
        <div class="pt-4">
          <VirtualScroller bind:items={selectedBook.reviews}>
            {#snippet renderer(post: Ref<Post>)}
              <div class="pb-4">
                <PostRenderer bind:post={post.ref} />
              </div>
            {/snippet}
          </VirtualScroller>
        </div>
      </div>
    </div>
  </div>
{/if} -->
