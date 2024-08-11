<script lang="ts">
  import CommentRenderer from "$components/CommentRenderer.svelte";
  import VirtualScroller from "$components/VirtualScroller.svelte";
  import type { Comment, Ref } from "$lib/types";

  const { data } = $props();

  const comments = $derived.by(() => {
    const state = $state(data.comments);
    return { state };
  });
</script>

{#snippet renderer(comment: Ref<Comment>)}
  <div class="pb-4">
    <CommentRenderer bind:comment={comment.ref} currentUser={data.user?.name} />
  </div>
{/snippet}
<VirtualScroller bind:items={comments.state} {renderer} />
