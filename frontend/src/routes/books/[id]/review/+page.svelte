<script lang="ts">
  import PostRenderer from "$components/renderers/PostRenderer.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
  import type { Post, Ref } from "$lib/types";
  import ReviewButton from "../../../posts/ReviewButton.svelte";

  const { data } = $props();

  let reviews = $state(data.book.reviews);

  $effect(() => {
    reviews = data.book.reviews;
  });
</script>

<h2>Reviews</h2>
<ReviewButton book={data.book} />
{#if reviews.length > 0}
  <VirtualScroller bind:items={reviews}>
    {#snippet renderer(post: Ref<Post>)}
      <div class="pb-6">
        <PostRenderer bind:post={post.ref} />
      </div>
    {/snippet}
  </VirtualScroller>
{:else}
  <p class="text-lg font-semibold leading-3 tracking-tight">There's no review yet!</p>
{/if}
