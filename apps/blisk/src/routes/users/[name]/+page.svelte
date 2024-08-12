<script lang="ts">
  import { page } from "$app/stores";
  import PostRenderer from "$components/PostRenderer.svelte";
  import VirtualScroller from "$components/VirtualScroller.svelte";
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
    const data = await fetchBackend<Post[]>(`/users/${$page.params.name}/posts?offset=${posts.state.length}`, {
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
