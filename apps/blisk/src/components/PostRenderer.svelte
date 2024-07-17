<script lang="ts">
  import type { Post } from "$lib/types";
  import Comment from "./icons/Comment.svelte";
  import Share from "./icons/Share.svelte";
  import ThumbUp from "./icons/ThumbUp.svelte";
  import PostRendererButton from "./PostRendererButton.svelte";

  interface PostRendererProps {
    post: Post;
  }

  const { post }: PostRendererProps = $props();
</script>

<article class="box flex flex-col gap-3 rounded-[31px] p-4 shadow-md">
  <h3 class="order-2">{post.title}</h3>
  <h4 class="order-1 flex flex-row gap-2">
    <img src="/no-avatar.webp" class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg" alt="" />
    <span class="flex flex-col gap-1">
      {post.author_name}
      <span class="text-comment text-sm">Just now</span>
    </span>
  </h4>
  <div class="order-3 text-base font-normal">{post.content}</div>
  <div class="-m-1 order-4 flex flex-row gap-3">
    <PostRendererButton as="div">
      <ThumbUp width={24} height={24} class="h-6 w-auto" />
      <span class="mb-[-1px]">Like</span>
    </PostRendererButton>
    <PostRendererButton as="a" href="/posts/{post.id}#comments">
      <Comment width={24} height={24} class="h-6 w-auto" />
      <span class="mb-[-1px]">Comment</span>
    </PostRendererButton>
    <PostRendererButton as="div">
      <Share width={24} height={24} class="h-6 w-auto" />
      <span class="mb-[-1px]">Share</span>
    </PostRendererButton>
  </div>
</article>
