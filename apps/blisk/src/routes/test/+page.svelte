<script lang="ts">
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

  // const emblaPrev = () => {
  //   if (!emblaApi) return;
  //   const { index } = emblaApi.internalEngine();
  //   const next = index.add(-10).get();
  //   emblaApi.scrollTo(next, false);
  // };

  // const emblaNext = () => {
  //   if (!emblaApi) return;
  //   const { index } = emblaApi.internalEngine();
  //   const next = index.add(10).get();
  //   emblaApi.scrollTo(next, false);
  // };
</script>

<div class="bg-wood w-full bg-[#d8a85f]">
  {#each Array.from({ length: 5 }) as _}
    <div
      class="relative w-full border-[8px] border-t-0 border-x-[#856940] border-b-[#6b5330] border-t-transparent px-4 shadow-[inset_0_0_20px_5px_#000000]"
    >
      <h2 class="py-4 text-[#050300]">science fiction</h2>
      <div
        class="mx-[-4px] mb-[-4px] overflow-x-hidden"
        use:emblaCarousel={{ options: { align: "start", containScroll: false, dragFree: true, loop: true }, plugins: [] }}
        onemblaInit={onEmbiaInit}
      >
        <div class="flex w-full flex-row">
          {#each Array.from({ length: 20 }) as _}
            <a href="/books/1" class="book-container z-[3] mr-10 shrink-0 select-none shadow-[5px_2px_20px_-1px_#000000] hover:z-[4]">
              <img src="/test-cover.jpg" width="192" height="288" alt="" class="h-auto w-48" />
            </a>
          {/each}
        </div>
      </div>
    </div>
    <div class="bg-wood h-[20px] bg-[#856940] last:hidden" aria-hidden="true"></div>
  {/each}
</div>
