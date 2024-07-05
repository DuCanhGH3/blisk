<!-- svelte-ignore deprecated_event_handler -->
<script lang="ts">
  import ChevronLeft from "$components/icons/ChevronLeft.svelte";
  import ChevronRight from "$components/icons/ChevronRight.svelte";
  import { clsx } from "$lib/clsx";
  import type { EmblaCarouselType } from "embla-carousel";
  import emblaCarousel from "embla-carousel-svelte";

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
    const { index } = emblaApi?.internalEngine();
    const next = index.add(-10).get();
    emblaApi.scrollTo(next, false);
  };

  const emblaNext = () => {
    if (!emblaApi) return;
    const { index } = emblaApi?.internalEngine();
    const next = index.add(10).get();
    emblaApi.scrollTo(next, false);
  };
</script>

<div class="h-full w-full">
  <h2 class="mb-2 text-4xl"><span class="sr-only">Category: </span>mystery thrillers</h2>
  <div class="relative w-full py-4">
    <button
      class={clsx(
        "absolute left-4 top-1/2 z-10 flex -translate-y-1/2 active:!bg-neutral-400 dark:active:!bg-neutral-700",
        "flex cursor-pointer items-center justify-center rounded-full bg-transparent [&>svg]:invisible [&>svg]:hover:visible",
        "hover:bg-neutral-250 p-3 transition-colors duration-100 dark:hover:bg-neutral-800"
      )}
      onclick={emblaPrev}
    >
      <ChevronLeft width={24} height={24} class="transition-all duration-100" />
    </button>
    <button
      class={clsx(
        "absolute right-4 top-1/2 z-10 flex -translate-y-1/2 active:!bg-neutral-400 dark:active:!bg-neutral-700",
        "flex cursor-pointer items-center justify-center rounded-full bg-transparent [&>svg]:invisible [&>svg]:hover:visible",
        "hover:bg-neutral-250 p-3 transition-colors duration-100 dark:hover:bg-neutral-800"
      )}
      onclick={emblaNext}
    >
      <ChevronRight width={24} height={24} class="transition-all duration-100" />
    </button>
    <div
      class="z-[2] mb-[-65px] flex h-full w-full overflow-x-hidden pb-[65px]"
      use:emblaCarousel={{ options: { align: "start", containScroll: false, dragFree: true, loop: true }, plugins: [] }}
      on:emblaInit={onEmbiaInit}
    >
      <div class="flex w-full flex-row">
        {#each Array.from({ length: 1 }) as _}
          <a href="/books/1" class="book-container z-[3] mr-1 shrink-0 hover:z-[4]">
            <span class="book">
              <span class="book-side spine">
                <img src="/test-spine.jpg" width="48" height="288" alt="" class="h-full w-full" />
              </span>
              <span class="book-side top"></span>
              <span class="book-side back-cover"></span>
              <span class="book-side cover">
                <img src="/test-cover.jpg" width="192" height="288" alt="" class="h-full w-full" />
              </span>
            </span>
          </a>
        {/each}
      </div>
    </div>
    <div class="dark:bg-neutral-925 z-[1] h-7 w-full bg-white shadow-[0_0_10px_2px_rgb(0,_0,_0,_0.5)]"></div>
  </div>
</div>
