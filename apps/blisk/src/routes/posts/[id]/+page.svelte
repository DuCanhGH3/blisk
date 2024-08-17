<script lang="ts">
  import { fetchBackend } from "$lib/backend.client.js";
  import type { Comment } from "$lib/types.js";
  import LargePostRenderer from "./LargePostRenderer.svelte";

  const { data } = $props();

  const post = $derived.by(() => {
    const state = $state(data.post);
    return { state };
  });
</script>

<LargePostRenderer
  bind:post={post.state}
  loadMoreComments={async () => {
    if (post.state.comments.length === 0) return [];
    const lastComment = post.state.comments[post.state.comments.length - 1];
    const data = await fetchBackend<Comment[]>(`/comments?post_id=${post.state.id}&previous_last=${lastComment.id}`, {
      signal: AbortSignal.timeout(10000),
    });
    if (!data.ok) {
      throw new Error(data.error);
    }
    return data.data;
  }}
/>
