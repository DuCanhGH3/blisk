<script lang="ts">
  import { page } from "$app/stores";
  import PostRenderer from "$components/renderers/PostRenderer.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
  import { fetchBackend } from "$lib/backend.client.js";
  import type { Post, Ref } from "$lib/types";

  const { data } = $props();

  let posts = $derived.by(() => {
    const state = $state(data.posts);
    return { state };
  });
</script>

<!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
<!-- svelte-ignore binding_property_non_reactive -->
<VirtualScroller
  bind:items={posts.state}
  loadMore={async () => {
    if (posts.state.length === 0) return [];
    const previousLast = posts.state[posts.state.length - 1];
    const data = await fetchBackend<Post[]>(`/users/${$page.params.name}/posts?previous_last=${previousLast.id}`, {
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
