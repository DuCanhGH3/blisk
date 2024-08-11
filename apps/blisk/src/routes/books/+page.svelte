<script lang="ts">
  import { clsx } from "$lib/clsx";

  const { data } = $props();
</script>

<div class="bg-wood h-fit w-full bg-[#d8a85f]">
  <ul
    class={clsx(
      "bookshelf relative mb-[-4px] grid w-full gap-10 border-x-[#856940] px-10",
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
          "relative select-none pt-10",
          "before:w-[calc(100%+2*(var(--gap)+var(--depth)))] before:translate-x-[calc(-1*var(--gap)-var(--depth))] before:translate-y-full",
          "before:absolute before:bottom-0 before:!border-t-0 before:border-transparent before:border-b-[#6b5330] before:[border-width:var(--depth)]",
          "after:bg-wood after:absolute after:bottom-0 after:h-5 after:w-[calc(100%+2*(var(--gap)+var(--depth)))] after:bg-[#d8a85f]",
          "after:translate-x-[calc(-1*var(--gap)-var(--depth))] after:translate-y-[calc(100%+var(--depth))] after:shadow-[0px_8px_20px_-2px_#000]"
        )}
      >
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
