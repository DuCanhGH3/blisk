<script lang="ts">
  import { fetchBackend } from "$lib/backend.client.js";
  import type { Comment } from "$lib/types.js";
  import LargePostRenderer from "./LargePostRenderer.svelte";

  const { data } = $props();

  let post = $state(data.post);

  $effect(() => {
    post = data.post;
  });
</script>

<LargePostRenderer
  bind:post
  loadMoreComments={async () => {
    if (post.comments.length === 0) return [];
    const lastComment = post.comments[post.comments.length - 1];
    const data = await fetchBackend<Comment[]>(`/comments?post_id=${post.id}&previous_last=${lastComment.id}`, {
      signal: AbortSignal.timeout(10000),
    });
    if (!data.ok) {
      throw new Error(data.error);
    }
    return data.data;
  }}
/>
