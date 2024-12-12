<script lang="ts">
  import { base } from "$app/paths";
  import { page } from "$app/stores";
  import LinkBox from "$components/LinkBox.svelte";
  import MarkdownRenderer from "$components/renderers/MarkdownRenderer.svelte";
  import TooltipHover from "$components/TooltipHover.svelte";
  import { clsx } from "$lib/clsx";
  import { getImage, percentageToBookRating } from "$lib/utils";
  import LinkButton from "./LinkButton.svelte";

  const { children, data } = $props();

  const fromCategory = $derived($page.url.searchParams.get("from-category"));
  const fromPost = $derived($page.url.searchParams.get("from-post"));
  const basePath = $derived<`/${string}`>(`${base}/books/${$page.params.id}`);
</script>

<div class="mx-auto flex h-full w-full flex-col gap-8 p-4 md:px-20 md:py-10">
  <div class="flex w-full flex-col gap-4 lg:flex-row lg:gap-8">
    <img
      src={getImage(data.book.cover_image, "/test-cover.jpg")}
      class="h-72 w-48 select-none shadow-[5px_2px_20px_-1px_#000000]"
      style:view-transition-name="book{fromCategory ? `-${fromCategory}` : ''}{fromPost ? `-${fromPost}` : ''}-{data.book.name}"
      width="192"
      height="288"
      loading="lazy"
      decoding="async"
      alt=""
    />
    <div
      class={clsx(
        "flex flex-1 flex-col gap-2 break-all",
        "[&>div]:flex [&>div]:flex-row [&>div]:flex-wrap [&>div]:items-center [&>div]:gap-2",
        "[&>div>h3]:text-lg [&>div>h3]:font-semibold [&>div>h3]:leading-3 [&>div>h3]:tracking-tight"
      )}
    >
      <h1 class="pt-4 text-5xl lg:text-8xl">{data.book.title}</h1>
      <h2 class="sr-only">About this book</h2>
      <p class="text-3xl font-semibold leading-10 tracking-tight">
        <span class="sr-only">Written by</span>
        {#each data.book.authors as author, idx}
          {@const last = idx === data.book.authors.length - 1}
          <a class="link no-color" href="/books?author={author.id}">{author.name}{last ? "" : ", "}</a>
        {/each} • 1965
      </p>
      <div class="max-h-72 overflow-y-auto">
        <MarkdownRenderer source={data.book.summary} startingHeading={3} />
      </div>
    </div>
  </div>
  <div class="flex flex-col justify-between gap-14 lg:flex-row-reverse">
    <section
      id="statistics"
      class={clsx(
        "flex h-fit w-fit basis-1/3 flex-col gap-6 overflow-x-auto lg:sticky lg:top-14",
        "[&>div>h3]:mb-3 [&>div>h3]:text-lg [&>div>h3]:font-bold [&>div>h3]:leading-3 [&>div>h3]:tracking-tight"
      )}
    >
      <h2>About {data.book.title}</h2>
      <div>
        <h3>Author</h3>
        <div class="flex flex-row flex-wrap gap-1">
          {#each data.book.authors as author}
            <LinkButton href="/books?author={author.id}">{author.name}</LinkButton>
          {/each}
        </div>
      </div>
      <div>
        <h3>Language</h3>
        <p>{data.book.language}</p>
      </div>
      <!-- <div>
        <h3>Series</h3>
        <p>Dune series</p>
      </div> -->
      <div>
        <h3>Genre</h3>
        <div class="flex flex-row flex-wrap gap-2">
          {#each data.book.categories as category}
            <LinkButton href="/books?category={category.id}">
              {category.name}
            </LinkButton>
          {/each}
        </div>
      </div>
      <div class="[&_p]:w-fit [&_p]:font-semibold">
        <h3>Reviews</h3>
        {#if !data.book.reactions}
          <p class="w-fit font-semibold">Not Available</p>
        {:else}
          {@const percentage = Math.round((data.book.reactions.like / data.book.reactions.total) * 100)}
          {@const bookRating = percentageToBookRating(percentage)}
          {@const isPositive = bookRating === "Overwhelmingly Positive" || bookRating === "Very Positive" || bookRating === "Positive"}
          {@const isMixed = bookRating === "Mixed"}
          {@const isNegative = !isPositive && !isMixed}
          <TooltipHover
            tooltipId="book-rating-tooltip"
            content={isNegative
              ? `${data.book.reactions.dislike} (${100 - percentage}%) out of ${data.book.reactions.total} reviews were negative`
              : `${data.book.reactions.like} (${percentage}%) out of ${data.book.reactions.total} reviews were positive`}
          >
            <p
              class={clsx(
                "w-fit font-semibold",
                isNegative ? "text-error-light dark:text-error-dark" : isPositive ? "text-accent-light dark:text-accent-dark" : ""
              )}
            >
              {bookRating}
              {#if isNegative}
                ({data.book.reactions.dislike} negative review{data.book.reactions.dislike !== 1 ? "s" : ""})
              {:else}
                ({data.book.reactions.like} positive review{data.book.reactions.like !== 1 ? "s" : ""})
              {/if}
            </p>
          </TooltipHover>
        {/if}
      </div>
    </section>
    <section id="reviews" class="basis-2/3">
      <div class="max-w-(--breakpoint-md) flex w-full flex-col gap-4">
        <div class="flex h-max w-full flex-row flex-wrap justify-stretch gap-2">
          <LinkBox href={basePath}>Read</LinkBox>
          <LinkBox href="{basePath}/review">Reviews</LinkBox>
          <LinkBox href="{basePath}/upload">Upload</LinkBox>
        </div>
        {@render children()}
      </div>
    </section>
  </div>
</div>
