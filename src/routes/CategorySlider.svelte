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
    emblaApi?.scrollPrev();
  };

  const emblaNext = () => {
    emblaApi?.scrollNext();
  };
</script>

<div class="h-full w-full">
  <h2 class="mb-2 text-4xl"><span class="sr-only">Category: </span>mystery thrillers</h2>
  <div class="relative w-full py-4">
    <button
      class={clsx(
        "absolute left-4 top-1/2 z-10 flex -translate-y-1/2 active:!bg-neutral-400 dark:active:!bg-neutral-700",
        "flex cursor-pointer items-center justify-center rounded-full bg-transparent [&>svg]:invisible [&>svg]:hover:visible",
        "p-3 transition-colors duration-100 hover:bg-neutral-250 dark:hover:bg-neutral-800"
      )}
      onclick={emblaPrev}
    >
      <ChevronLeft width={24} height={24} class="transition-all duration-100" />
    </button>
    <button
      class={clsx(
        "absolute right-4 top-1/2 z-10 flex -translate-y-1/2 active:!bg-neutral-400 dark:active:!bg-neutral-700",
        "flex cursor-pointer items-center justify-center rounded-full bg-transparent [&>svg]:invisible [&>svg]:hover:visible",
        "p-3 transition-colors duration-100 hover:bg-neutral-250 dark:hover:bg-neutral-800"
      )}
      onclick={emblaNext}
    >
      <ChevronRight width={24} height={24} class="transition-all duration-100" />
    </button>
    <div
      class="flex h-full w-full overflow-hidden"
      use:emblaCarousel={{ options: { align: "start", containScroll: false, dragFree: true, loop: true }, plugins: [] }}
      on:emblaInit={onEmbiaInit}
    >
      <div class="flex w-full">
        {#each Array.from({ length: 10 }) as _}
          <a
            href="/"
            class={clsx(
              "mr-4 flex w-full min-w-0 flex-[0_0_45%] select-none flex-col gap-4 rounded-[20px] border p-4 md:flex-[0_0_20%]",
              "border-neutral-300 bg-white dark:border-neutral-800 dark:bg-neutral-950"
            )}
          >
            <span class="p-2">
              <h3 class="text-comment line-clamp-1 text-xl font-semibold">recommendation</h3>
              <h4 class="line-clamp-1 text-3xl font-semibold">Lorem ipsum</h4>
            </span>
            <img
              width={900}
              height={400}
              class="h-64 w-full rounded-md object-cover opacity-75 md:h-[300px]"
              src="https://images.ctfassets.net/kftzwdyauwt9/44csSCT2TZUSqqI2UCLDF9/153e0192aeb75b2322007085c1009bc0/AGI2.png?w=3840&q=90&fm=webp"
              alt=""
            />
          </a>
        {/each}
      </div>
    </div>
  </div>
</div>
