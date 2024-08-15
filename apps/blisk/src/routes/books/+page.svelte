<script lang="ts">
  // TODO(ducanhgh): Work-in-progress.
  // import { goto, pushState } from "$app/navigation";
  // import { page } from "$app/stores";
  // import X from "$components/icons/X.svelte";
  // import PostRenderer from "$components/PostRenderer.svelte";
  // import { rendererButtonAttributes } from "$components/renderer-constants";
  // import VirtualScroller from "$components/VirtualScroller.svelte";
  // import Button from "$components/Button.svelte";
  import { clsx } from "$lib/clsx";
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
  <div class="bg-wood h-fit w-full bg-[#d8a85f]">
    <ul
      class={clsx(
        "relative mb-[-4px] grid w-full gap-[4.25rem] border-x-[#856940] px-10 pt-10",
        "!border-t-0 border-b-[#6b5330] border-t-transparent shadow-[inset_0_0_20px_5px_#000000]",
        "[border-width:var(--depth)] [grid-template-columns:repeat(auto-fit,minmax(min(12rem,100%),1fr))]",
        "after:bg-wood after:h-5 after:w-[calc(100%+2*var(--depth))] after:bg-[#d8a85f]",
        "after:absolute after:bottom-0 after:translate-x-[calc(-1*var(--depth))] after:translate-y-[calc(100%+var(--depth))]"
      )}
      style="--gap:2.5rem;--depth:0.5rem"
    >
      {#each data.books as book}
        <li
          class={clsx(
            "relative select-none",
            "before:w-[calc(100%+2*(var(--gap)+var(--depth)))] before:translate-x-[calc(-1*var(--gap)-var(--depth))] before:translate-y-full",
            "before:absolute before:bottom-0 before:!border-t-0 before:border-transparent before:border-b-[#6b5330] before:[border-width:var(--depth)]",
            "after:bg-wood after:absolute after:bottom-0 after:h-5 after:w-[calc(100%+2*(var(--gap)+var(--depth)))] after:bg-[#d8a85f]",
            "after:translate-x-[calc(-1*var(--gap)-var(--depth))] after:translate-y-[calc(100%+var(--depth))] after:shadow-[5px_8px_20px_-2px_rgb(0_0_0/60%)]"
          )}
        >
          <!-- <a href="/books/{book.name}" onclick={(e: MouseEventEvent<HTMLAnchorElement>) => showModal(e, book.name)}> -->
          <a href="/books/{book.name}">
            <img
              src="/test-cover.jpg"
              width="192"
              height="288"
              alt=""
              class="-mx-1 -mb-1 h-auto w-48 shadow-[5px_2px_20px_-1px_#000]"
              style="view-transition-name:book-{book.name}"
            />
            <span class="sr-only">Book {book.title}</span>
          </a>
        </li>
      {/each}
    </ul>
  </div>
{:else}
  <div class="flex flex-col gap-6 p-2 md:py-8">
    <h1 class="h2">No books found!</h1>
    <h2 class="h3">It's time to fill the bookshelf, isn't it?</h2>
  </div>
{/if}
<!-- {#if selectedBook}
  <div class="fixed left-0 top-0 z-50 flex h-full w-full items-center justify-center bg-black/80">
    <div class="box lg relative flex w-[90dvw] max-w-screen-lg flex-col gap-4 !p-8 text-black lg:flex-row lg:gap-8 dark:text-white">
      <Button as="button" variant="light" class="absolute right-4 top-4 z-[2] !p-2" onclick={() => history.back()}>
        <X {...rendererButtonAttributes} />
        <span class="sr-only">Go back</span>
      </Button>
      <img src="/test-cover.jpg" class="h-72 w-48 rounded-lg" width="192" height="288" alt="" />
      <div class="z-[1] flex flex-1 flex-col gap-2 break-all">
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
