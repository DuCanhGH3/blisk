<script lang="ts">
  import type { Post, ReactionType } from "$lib/types";
  import Comment from "./icons/Comment.svelte";
  import Share from "./icons/Share.svelte";
  import ThumbUp from "./icons/ThumbUp.svelte";
  import type { IconProps } from "./icons/types";
  import PostRendererButton from "./PostRendererButton.svelte";
  import ReactionBar from "./ReactionBar.svelte";
  import { reactionRender } from "./renderer-constants";

  interface PostRendererProps {
    post: Post;
  }

  const { post }: PostRendererProps = $props();

  let previousReaction: ReactionType | null = null;
  let currentReaction = $state<ReactionType | null>(post.user_reaction);
  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const rendererButtonAttributes = {
    width: 24,
    height: 24,
    class: "h-6 w-auto",
    "aria-hidden": "true",
    tabindex: -1,
  } satisfies IconProps;
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
  <div class="order-4 -m-1 flex flex-row gap-3">
    <details bind:this={reactionBar} class="relative flex-1">
      {#if !currentReaction}
        <PostRendererButton as="summary" aria-describedby="reaction-bar-{post.id}">
          <ThumbUp {...rendererButtonAttributes} /> <span class="mb-[-1px]">Like</span>
        </PostRendererButton>
      {:else}
        {@const { icon, label, colors } = reactionRender[currentReaction]}
        <PostRendererButton customColors={colors} as="summary" aria-describedby="reaction-bar-{post.id}">
          <svelte:component this={icon} animatable={false} {...rendererButtonAttributes} />
          <span class="mb-[-1px] text-black dark:text-white">{label}</span>
        </PostRendererButton>
      {/if}
      <ReactionBar
        id="reaction-bar-{post.id}"
        class="animate-fly absolute top-0 translate-y-[calc(-100%-4px)]"
        style="--fly-translate:0.25rem"
        forId={post.id}
        forType="post"
        updateReaction={(reaction) => {
          previousReaction = currentReaction;
          currentReaction = reaction;
          if (reactionBar) {
            reactionBar.open = false;
          }
        }}
        revertReaction={() => {
          currentReaction = previousReaction;
          previousReaction = null;
        }}
      />
    </details>
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
