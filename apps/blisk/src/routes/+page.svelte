<script lang="ts">
  import ChevronLeft from "$components/icons/ChevronLeft.svelte";
  import ChevronRight from "$components/icons/ChevronRight.svelte";
  import { clsx } from "$lib/clsx";
  import type { EmblaCarouselType } from "embla-carousel";
  import emblaAutoplay from "embla-carousel-autoplay";
  import emblaCarouselSvelte from "embla-carousel-svelte";
  import CategorySlider from "./CategorySlider.svelte";

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

<div class="flex w-full flex-col justify-center gap-10 self-stretch">
  <div class="relative h-full w-full">
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
      class={clsx(
        "flex h-full w-full overflow-hidden rounded-[21px] border shadow-md transition-colors duration-100",
        "border-border-light bg-white dark:border-border-dark dark:bg-neutral-915 hover:bg-neutral-100 dark:hover:bg-neutral-800"
      )}
      use:emblaCarouselSvelte={{ options: { containScroll: false, dragFree: true, loop: true }, plugins: [emblaAutoplay()] }}
      onemblaInit={onEmbiaInit}
    >
      <div class="flex w-full">
        {#each Array.from({ length: 3 }) as _}
          <a href="/" class="flex w-full min-w-0 flex-[0_0_100%] select-none flex-col gap-4 p-4">
            <span class="p-2">
              <h2 class="text-comment text-2xl font-semibold">book of the day</h2>
              <h3 class="line-clamp-1 text-4xl font-semibold">Lorem ipsum</h3>
            </span>
            <img
              width={900}
              height={400}
              class="h-96 w-full rounded-md object-cover opacity-75 md:h-[600px]"
              src="/AGI2.webp"
              alt=""
            />
          </a>
        {/each}
      </div>
    </div>
  </div>
  {#each Array.from({ length: 1 }) as _}
    <CategorySlider />
  {/each}
</div>
