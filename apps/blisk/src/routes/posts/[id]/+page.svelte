<script lang="ts">
  import CommentRenderer from "./CommentRenderer.svelte";
  import CommentForm from "./CommentForm.svelte";

  const { data } = $props();
</script>

<div class="flex w-full max-w-6xl flex-col gap-3 p-2 md:py-8">
  <article class="flex flex-col gap-3">
    <h1 class="order-2">{data.post.title}</h1>
    <h2 class="text-comment order-1 text-xl">{data.post.author_name}</h2>
    <div class="order-3 mt-4">
      {data.post.content}
    </div>
  </article>
  <CommentForm parentId={null} updateComments={(newComment) => data.comments.unshift(newComment)} />
  <ul id="comments">
    {#each data.comments as comment}
      <CommentRenderer {comment} username={data.user?.name} />
    {/each}
  </ul>
</div>
