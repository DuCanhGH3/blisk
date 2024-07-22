<script lang="ts">
  import CommentRenderer from "$components/CommentRenderer.svelte";
  import CommentForm from "$components/CommentForm.svelte";
  import MarkdownRenderer from "$components/MarkdownRenderer.svelte";
  import { clsx } from "$lib/clsx";

  const { data } = $props();

  let comments = $state(data.comments);
</script>

<article class="flex w-full max-w-6xl flex-col gap-8 p-2 md:py-8">
  <div class="flex flex-col gap-8">
    <h1 class="-order-1">{data.post.title}</h1>
    <div class="-order-2 flex flex-row items-center gap-4 font-semibold leading-10 tracking-tight">
      <img src="/no-avatar.webp" class="border-border-light dark:border-border-dark size-12 select-none rounded-full border shadow-lg" alt="" />
      <div class="flex flex-col">
        <div class="text-comment flex flex-row items-center gap-1 text-base">
          <a href="/users/{data.post.author_name}" class="link sm -mt-[1px]">{data.post.author_name}</a>
          <span>•</span>
          <div>Just now</div>
        </div>
        <div class={clsx("text-lg", data.post.reaction === "like" ? "text-accent-light dark:text-accent-dark" : "text-error")}>
          {data.post.reaction === "like" ? "Recommended" : "Not recommended"}
        </div>
      </div>
    </div>
    <MarkdownRenderer source={data.post.content} />
  </div>
  <section id="comments" class="flex flex-col gap-3">
    <CommentForm parentId={null} updateComments={(newComment) => comments.unshift(newComment)} />
    {#each comments as comment (comment.id)}
      <CommentRenderer {comment} username={data.user?.name} />
    {/each}
  </section>
</article>
