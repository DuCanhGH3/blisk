<script lang="ts">
  import { page } from "$app/stores";
  import CommentRenderer from "$components/renderers/CommentRenderer.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
  import { fetchBackend } from "$lib/backend.client.js";
  import type { Comment, Ref } from "$lib/types";

  const { data } = $props();

  let comments = $state(data.comments);

  $effect(() => {
    comments = data.comments;
  });
</script>

{#if comments.length > 0}
  <!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
  <!-- svelte-ignore binding_property_non_reactive -->
  <VirtualScroller
    bind:items={comments}
    loadMore={async () => {
      if (comments.length === 0) return [];
      const item = comments[comments.length - 1];
      const data = await fetchBackend<Comment[]>(`/comments?user=${$page.params.name}&previous_last=${item.id}`, {
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
            const oldComments = [...comments];
            comments = comments.filter((comment) => comment.id !== commentToFilter.id);
            return () => {
              comments = oldComments;
            };
          }}
        />
      </div>
    {/snippet}
  </VirtualScroller>
{:else}
  <div class="flex flex-col gap-4">
    <h3>User has not commented.</h3>
    <h4>Not the most talkative one, are they?</h4>
  </div>
{/if}
