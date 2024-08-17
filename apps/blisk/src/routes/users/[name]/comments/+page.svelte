<script lang="ts">
  import { page } from "$app/stores";
  import CommentRenderer from "$components/renderers/CommentRenderer.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
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
    if (comments.state.length === 0) return [];
    const item = comments.state[comments.state.length - 1];
    const data = await fetchBackend<Comment[]>(`/users/${$page.params.name}/comments?previous_last=${item.id}`, {
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
      <CommentRenderer
        bind:comment={comment.ref}
        depth={0}
        removeComment={(commentToFilter) => {
          const oldComments = [...comments.state];
          comments.state = comments.state.filter((comment) => comment.id !== commentToFilter.id);
          return () => {
            comments.state = oldComments;
          };
        }}
      />
    </div>
  {/snippet}
</VirtualScroller>
