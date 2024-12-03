<script lang="ts">
  import { page } from "$app/stores";
  import PostRenderer from "$components/renderers/PostRenderer.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
  import { fetchBackend } from "$lib/backend.client.js";
  import type { Post, Ref } from "$lib/types";

  const { data } = $props();

  let posts = $state(data.posts);

  $effect(() => {
    posts = data.posts;
  });
</script>

{#if posts.length > 0}
  <!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
  <!-- svelte-ignore binding_property_non_reactive -->
  <VirtualScroller
    bind:items={posts}
    loadMore={async () => {
      if (posts.length === 0) return [];
      const previousLast = posts[posts.length - 1];
      const data = await fetchBackend<Post[]>(`/posts?user=${$page.params.name}&previous_last=${previousLast.id}`, {
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
{:else}
  <div class="flex flex-col gap-4">
    <h3>User has not reviewed.</h3>
    <h4>It is time for them to read more, yeah?</h4>
  </div>
{/if}
