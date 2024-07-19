<script lang="ts">
  import CommentRenderer from "$components/CommentRenderer.svelte";
  import CommentForm from "$components/CommentForm.svelte";
  import { fly } from "svelte/transition";
  import { quintOut } from "svelte/easing";

  const { data } = $props();

  let comments = $state(data.comments);
</script>

<div class="flex w-full max-w-6xl flex-col gap-3 p-2 md:py-8">
  <article class="flex flex-col gap-3">
    <h1 class="order-2">{data.post.title}</h1>
    <h2 class="text-comment order-1 text-xl">{data.post.author_name}</h2>
    <div class="order-3 mt-4">
      {data.post.content}
    </div>
  </article>
  <CommentForm parentId={null} updateComments={(newComment) => comments.unshift(newComment)} />
  <ul class="flex flex-col gap-3" id="comments">
    {#each comments as comment (comment.id)}
      <li in:fly={{ duration: 100, easing: quintOut, y: -25 }}>
        <CommentRenderer {comment} username={data.user?.name} />
      </li>
    {/each}
  </ul>
</div>
