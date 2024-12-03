<script lang="ts">
  import ChevronLeft from "$components/icons/ChevronLeft.svelte";
  import ChevronRight from "$components/icons/ChevronRight.svelte";
  import { clsx } from "$lib/clsx";
  import type { BookCategoryWithBooks } from "$lib/types";
  import { getImage } from "$lib/utils";
  import type { EmblaCarouselType } from "embla-carousel";
  import emblaCarousel from "embla-carousel-svelte";

  const { category }: { category: BookCategoryWithBooks } = $props();

  let emblaApi = $state<EmblaCarouselType | null>(null);

  const onEmbiaInit = (event: CustomEvent<EmblaCarouselType>) => {
    emblaApi = event.detail;
    emblaApi.on("pointerUp", (emblaApi) => {
      const { scrollTo, target, location } = emblaApi.internalEngine();
      const diffToTarget = target.get() - location.get();
      scrollTo.distance(diffToTarget * 0.1, true);
    });
  };

  const emblaPrev = () => {
    if (!emblaApi) return;
    const { index } = emblaApi.internalEngine();
    const next = index.add(-10).get();
    emblaApi.scrollTo(next, false);
  };

  const emblaNext = () => {
    if (!emblaApi) return;
    const { index } = emblaApi.internalEngine();
    const next = index.add(10).get();
    emblaApi.scrollTo(next, false);
  };
</script>

<div class="w-full">
  <h2 class="sr-only py-5">Category: {category.name}</h2>
  <div class="absolute h-full w-full py-4" role="presentation">
    <button
      class={clsx(
        "absolute left-4 top-1/2 z-10 flex -translate-y-1/2 active:!bg-neutral-400 dark:active:!bg-neutral-700",
        "flex cursor-pointer items-center justify-center rounded-full bg-transparent [&>svg]:invisible [&>svg]:hover:visible",
        "hover:bg-neutral-250 p-3 transition-colors duration-100 dark:hover:bg-neutral-800"
      )}
      onclick={emblaPrev}
    >
      <ChevronLeft width={24} height={24} class="transition-all duration-100" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Go to previous page of category: {category.name}</span>
    </button>
    <button
      class={clsx(
        "absolute right-4 top-1/2 z-10 flex -translate-y-1/2 active:!bg-neutral-400 dark:active:!bg-neutral-700",
        "flex cursor-pointer items-center justify-center rounded-full bg-transparent [&>svg]:invisible [&>svg]:hover:visible",
        "hover:bg-neutral-250 p-3 transition-colors duration-100 dark:hover:bg-neutral-800"
      )}
      onclick={emblaNext}
    >
      <ChevronRight width={24} height={24} class="transition-all duration-100" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Go to next page of category: {category.name}</span>
    </button>
  </div>
  <div
    class={clsx(
      "relative w-full border-[8px] border-t-0 border-t-transparent px-10 shadow-[inset_0_0_20px_5px_#000000]",
      "border-x-wood-650 border-b-wood-700 dark:border-x-wood-915 dark:border-b-wood-1000"
    )}
  >
    <span class="h2 block py-5" aria-hidden="true">{category.name}</span>
    <div
      class="mx-[-4px] mb-[-4px] overflow-x-hidden"
      use:emblaCarousel={{ options: { align: "start", containScroll: false, dragFree: true, loop: true }, plugins: [] }}
      onemblaInit={onEmbiaInit}
    >
      <div class="flex w-full flex-row">
        {#each category.books as book}
          <a
            href="/books/{book.name}?from-category={category.name}"
            class="book-container z-[3] mr-10 shrink-0 select-none shadow-[5px_2px_20px_-1px_#000000] hover:z-[4]"
          >
            <img
              src={getImage(book.cover_image, "/test-cover.jpg")}
              width="192"
              height="288"
              alt="Book: {book.title}"
              class="h-72 w-48 object-fill"
              style="view-transition-name:book-{category.name}-{book.name}"
            />
          </a>
        {/each}
      </div>
    </div>
  </div>
</div>
<div class="bg-wood bg-wood-500 dark:bg-wood-950 dark:bg-dark-wood h-[20px] last:hidden" aria-hidden="true"></div>
