<script lang="ts">
  import LargePostRenderer from "../../LargePostRenderer.svelte";

  const { data } = $props();

  let post = $state(data.post);

  $effect(() => {
    post = data.post;
  });
</script>

<LargePostRenderer bind:post showCommentForm={false} />
