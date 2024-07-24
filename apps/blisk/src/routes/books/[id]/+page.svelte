<script lang="ts">
  import PostRenderer from "$components/PostRenderer.svelte";
  import { clsx } from "$lib/clsx.js";

  const { data } = $props();
</script>

<div class="mx-auto flex h-full w-full max-w-screen-lg flex-col gap-4 p-4 md:px-24">
  <div class="flex w-full flex-col gap-4 lg:h-72 lg:flex-row lg:gap-8">
    <div class="book-container">
      <span class="book front">
        <span class="book-side spine">
          <img src="/test-spine.jpg" width="48" height="288" alt="" class="h-full w-full" />
        </span>
        <span class="book-side top"></span>
        <span class="book-side back-cover"></span>
        <span class="book-side cover">
          <img src="/test-cover.jpg" width="192" height="288" alt="" class="h-full w-full" />
        </span>
      </span>
    </div>
    <div class="flex flex-1 flex-col gap-2">
      <h1 class="pt-4 text-5xl">{data.book.title}</h1>
      <h2 class="sr-only">About this book</h2>
      <p class="text-comment text-3xl font-semibold leading-10 tracking-tight" aria-label="This book was written by Frank Herbert in 1965">
        Frank Herbert • 1965
      </p>
      <p class="max-h-72 overflow-y-auto lg:max-h-full">
        {data.book.summary}
      </p>
    </div>
  </div>
  <h2 class="sr-only">Statistics</h2>
  <div class="w-full overflow-x-auto">
    <table class="w-full table-auto border-separate border-spacing-4">
      <tbody class="[&>tr>th]:text-left">
        <tr>
          <th>Author</th>
          <td>Frank Herbert</td>
        </tr>
        <tr>
          <th>Language</th>
          <td>English</td>
        </tr>
        <tr>
          <th>Series</th>
          <td>Dune series</td>
        </tr>
        <tr>
          <th>Genre</th>
          <td class="flex flex-row flex-wrap gap-1">
            {#each data.book.categories as category}
              <a
                class={clsx(
                  "rounded-md px-2 transition-colors duration-100",
                  "dark:hover:bg-accent-dark bg-neutral-250 hover:bg-accent-light hover:text-white dark:bg-neutral-800 dark:hover:text-black"
                )}
                href="/books/categories/{category.id}"
              >
                {category.name}
              </a>
            {/each}
          </td>
        </tr>
        <tr>
          <th>Pages</th>
          <td>412</td>
        </tr>
        <tr>
          <th>Recent reviews</th>
          <td>Overwhelmingly Positive</td>
        </tr>
        <tr>
          <th>All reviews</th>
          <td>Overwhelmingly Positive</td>
        </tr>
      </tbody>
    </table>
  </div>
  <!-- <h2>Buy this book</h2> -->
  <h2>Reviews</h2>
  {#each data.book.reviews as post}
    <PostRenderer {post} />
  {/each}
</div>
