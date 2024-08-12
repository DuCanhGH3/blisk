<script lang="ts">
  import { page } from "$app/stores";
  import CommentRenderer from "$components/CommentRenderer.svelte";
  import VirtualScroller from "$components/VirtualScroller.svelte";
  import { fetchBackend } from "$lib/backend.client.js";
  import type { Comment, Ref } from "$lib/types";

  const { data } = $props();

  const comments = $derived.by(() => {
    const state = $state(data.comments);
    return { state };
  });
</script>

<!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
<!-- svelte-ignore binding_property_non_reactive -->
<VirtualScroller
  bind:items={comments.state}
  loadMore={async () => {
    const data = await fetchBackend<Comment[]>(`/users/${$page.params.name}/comments?offset=${comments.state.length}`, {
      signal: AbortSignal.timeout(10000),
    });
    if (!data.ok) {
      throw new Error(data.error);
    }
    return data.data;
  }}
>
  {#snippet renderer(comment: Ref<Comment>)}
    <div class="pb-4">
      <!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
      <!-- svelte-ignore binding_property_non_reactive -->
      <CommentRenderer bind:comment={comment.ref} currentUser={data.user?.name} />
    </div>
  {/snippet}
</VirtualScroller>
