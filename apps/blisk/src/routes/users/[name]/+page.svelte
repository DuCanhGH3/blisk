<script lang="ts">
  import PostRenderer from "$components/PostRenderer.svelte";
  import VirtualScroller from "$components/VirtualScroller.svelte";
  import type { Post, Ref } from "$lib/types";

  const { data } = $props();

  let posts = $derived.by(() => {
    const state = $state(data.posts);
    return { state };
  });
</script>

{#snippet renderer(post: Ref<Post>)}
  <div class="pb-4">
    <PostRenderer bind:post={post.ref} />
  </div>
{/snippet}
<VirtualScroller bind:items={posts.state} {renderer} />
