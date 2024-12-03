<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { Book } from "$lib/types";
  import { getImage } from "$lib/utils";

  const { books }: { books: Book[] } = $props();
</script>

<div class="bg-wood dark:bg-dark-wood bg-wood-500 dark:bg-wood-950 h-fit w-full">
  <ul
    class={clsx(
      "relative mb-[-4px] grid w-full gap-[4.25rem] px-10 pt-10 shadow-[inset_0_0_20px_5px_#000000]",
      "border-x-wood-650 border-b-wood-700 dark:border-x-wood-915 dark:border-b-wood-1000 !border-t-0 border-t-transparent",
      "[border-width:var(--depth)] [grid-template-columns:repeat(auto-fit,minmax(min(12rem,100%),1fr))]",
      "after:bg-wood after:bg-wood-500 after:dark:bg-wood-950 after:dark:bg-dark-wood after:h-5 after:w-[calc(100%+2*var(--depth))]",
      "after:absolute after:bottom-0 after:translate-x-[calc(-1*var(--depth))] after:translate-y-[calc(100%+var(--depth))]"
    )}
    style="--gap:2.5rem;--depth:0.5rem"
  >
    {#each books as book}
      <li
        class={clsx(
          "relative select-none",
          "before:w-[calc(100%+2*(var(--gap)+var(--depth)))] before:translate-x-[calc(-1*var(--gap)-var(--depth))] before:translate-y-full",
          "before:border-b-wood-700 dark:before:border-b-wood-1000 before:border-transparent",
          "before:absolute before:bottom-0 before:!border-t-0 before:[border-width:var(--depth)]",
          "after:bg-wood dark:after:bg-dark-wood after:bg-wood-500 after:dark:bg-wood-950 after:shadow-[5px_8px_20px_-2px_rgb(0_0_0/60%)]",
          "after:absolute after:bottom-0 after:h-5 after:w-[calc(100%+2*(var(--gap)+var(--depth)))]",
          "after:translate-x-[calc(-1*var(--gap)-var(--depth))] after:translate-y-[calc(100%+var(--depth))]"
        )}
      >
        <!-- <a href="/books/{book.name}" onclick={(e: MouseEventEvent<HTMLAnchorElement>) => showModal(e, book.name)}> -->
        <a href="/books/{book.name}">
          <img
            src={getImage(book.cover_image, "/test-cover.jpg")}
            width="192"
            height="288"
            alt=""
            class="-mx-1 -mb-1 h-72 w-48 object-fill shadow-[5px_2px_20px_-1px_#000]"
            style="view-transition-name:book-{book.name}"
          />
          <span class="sr-only">Book {book.title}</span>
        </a>
      </li>
    {/each}
  </ul>
</div>
