<script lang="ts">
  import ChevronLeft from "$components/icons/ChevronLeft.svelte";
  import ChevronRight from "$components/icons/ChevronRight.svelte";
  import { clsx } from "$lib/clsx";
  import type { EmblaCarouselType } from "embla-carousel";
  import emblaAutoplay from "embla-carousel-autoplay";
  import emblaCarouselSvelte from "embla-carousel-svelte";
  import CategorySlider from "./CategorySlider.svelte";

  const { data } = $props();

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
    emblaApi?.scrollPrev();
  };

  const emblaNext = () => {
    emblaApi?.scrollNext();
  };
</script>

<div class="flex w-full flex-col gap-10 self-stretch">
  <h1 class="sr-only">Home</h1>
  <section id="book-of-the-day" class="relative h-full w-full">
    <h2 class="sr-only">Books of the day</h2>
    <button
      class={clsx(
        "absolute left-4 top-1/2 z-10 flex -translate-y-1/2 active:!bg-neutral-400 dark:active:!bg-neutral-700",
        "flex cursor-pointer items-center justify-center rounded-full bg-transparent [&>svg]:invisible [&>svg]:hover:visible",
        "hover:bg-neutral-250 p-3 transition-colors duration-100 dark:hover:bg-neutral-800"
      )}
      onclick={emblaPrev}
    >
      <ChevronLeft width={24} height={24} class="transition-all duration-100" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Go to previous page</span>
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
      <span class="sr-only">Go to next page</span>
    </button>
    <div
      class={clsx(
        "relative w-full overflow-hidden border-[8px] border-t-0 border-t-transparent shadow-[inset_0_0_20px_5px_#000000]",
        "border-x-wood-650 border-b-wood-700 dark:border-x-wood-915 dark:border-b-wood-1000"
      )}
      use:emblaCarouselSvelte={{ options: { containScroll: false, dragFree: true, loop: true }, plugins: [emblaAutoplay()] }}
      onemblaInit={onEmbiaInit}
    >
      <div class="flex w-full">
        {#each Array.from({ length: 3 }) as _}
          <a href="/" class="flex w-full min-w-0 flex-[0_0_100%] select-none flex-col gap-4 p-4">
            <span class="p-2">
              <span class="text-comment text-2xl font-semibold" aria-hidden="true">book of the day</span>
              <h3 class="line-clamp-1 text-4xl font-semibold"><span class="sr-only">Book: </span>Lorem ipsum</h3>
            </span>
            <img width={900} height={400} class="h-96 w-full rounded-md object-cover opacity-75 md:h-[600px]" src="/AGI2.webp" alt="" />
          </a>
        {/each}
      </div>
    </div>
  </section>
  <section id="book-recommendations" class="bg-wood dark:bg-dark-wood bg-wood-500 dark:bg-wood-950 w-full">
    <h2 class="sr-only">Recommended for you</h2>
    {#each data.categories as category}
      {#if category.books.length > 0}
        <CategorySlider {category} />
      {/if}
    {/each}
  </section>
</div>
