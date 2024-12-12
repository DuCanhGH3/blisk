<script lang="ts">
  import PostRenderer from "$components/renderers/PostRenderer.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
  import { fetchBackend } from "$lib/backend.client.js";
  import type { Post, Ref } from "$lib/types";
  import ReviewButton from "./ReviewButton.svelte";

  const { data } = $props();

  let posts = $state(data.posts);

  $effect(() => {
    posts = data.posts;
  });
</script>

<div class="max-w-(--breakpoint-md) mx-auto flex w-full flex-col gap-8 p-4 md:p-10">
  <ReviewButton />
  <section id="posts">
    <VirtualScroller
      bind:items={posts}
      loadMore={async () => {
        if (posts.length === 0) return [];
        const previousLast = posts[posts.length - 1];
        const data = await fetchBackend<Post[]>(`/posts?previous_last=${previousLast.id}`, {
          signal: AbortSignal.timeout(10000),
        });
        if (!data.ok) {
          throw new Error(data.error);
        }
        return data.data;
      }}
    >
      {#snippet renderer(post: Ref<Post>)}
        <div class="pb-4">
          <!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
          <!-- svelte-ignore binding_property_non_reactive -->
          <PostRenderer bind:post={post.ref} />
        </div>
      {/snippet}
    </VirtualScroller>
  </section>
</div>
<!-- {#if $page.state.createPost}
  <div class="fixed left-0 top-0 z-50 flex h-full w-full items-center justify-center bg-black/80">
    <CreateBox />
  </div>
{/if} -->
