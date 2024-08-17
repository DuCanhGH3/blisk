<script lang="ts">
  import LargePostRenderer from "../../LargePostRenderer.svelte";

  const { data } = $props();

  const post = $derived.by(() => {
    const state = $state(data.post);
    return { state };
  });
</script>

<LargePostRenderer bind:post={post.state} showCommentForm={false} />
