<script lang="ts">
  // Component is (mostly) stateless so that it works with the virtual scroller.
  import type { Post, ReactionType } from "$lib/types";
  import Comment from "./icons/Comment.svelte";
  import Share from "./icons/Share.svelte";
  import ThumbUp from "./icons/ThumbUp.svelte";
  import ThumbUpFilled from "./icons/ThumbUpFilled.svelte";
  import MarkdownRenderer from "./MarkdownRenderer.svelte";
  import PostRendererButton from "./PostRendererButton.svelte";
  import ReactionBar from "./ReactionBar.svelte";
  import { rendererButtonAttributes, reactionRender } from "./renderer-constants";
  import TooltipHover from "./TooltipHover.svelte";

  interface PostRendererProps {
    /**
     * The post to be rendered. Must be a state for the component to work properly.
     */
    post: Post;
  }

  const { post = $bindable() }: PostRendererProps = $props();

  let previousReaction: ReactionType | null = null;

  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const updateReaction = (reaction: ReactionType | null) => {
    previousReaction = post.user_reaction;
    post.user_reaction = reaction;
  };
</script>

<article class="box flex flex-col gap-3 rounded-[31px] p-4 shadow-md">
  <h3 class="h1 -order-1 mb-4">{post.title}</h3>
  <div class="-order-2 flex flex-row flex-wrap gap-2 font-semibold leading-10 tracking-tight">
    <img src="/no-avatar.webp" class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg" alt="" />
    <div>
      <div class="text-comment flex flex-row flex-wrap items-center gap-1 text-sm">
        <a href="/users/{post.author_name}" class="link sm">{post.author_name}</a>
        <span>•</span>
        <TooltipHover tooltipId="post-{post.id}-timestamp-tooltip" content="Just now">Just now</TooltipHover>
      </div>
      <div class="flex flex-row flex-wrap items-center gap-1 text-base">
        {#if post.reaction === "like"}
          <ThumbUpFilled width={20} height={20} class="fill-accent-light dark:fill-accent-dark h-auto w-5" aria-hidden tabindex={-1} />
          <span class="text-accent-light dark:text-accent-dark">Recommended</span>
        {:else}
          <ThumbUpFilled width={20} height={20} class="fill-error-light dark:fill-error-dark h-auto w-5 -scale-y-100" aria-hidden tabindex={-1} />
          <span class="text-error-light dark:text-error-dark">Not recommended</span>
        {/if}
      </div>
    </div>
  </div>
  <MarkdownRenderer source={post.content} startingHeading={4} />
  <div class="order-1 -m-1 flex flex-row flex-wrap gap-3">
    <details bind:this={reactionBar} class="relative flex-grow">
      {#if !post.user_reaction}
        <PostRendererButton as="summary" aria-describedby="post-{post.id}-reaction-bar">
          <ThumbUp {...rendererButtonAttributes} /> <span class="mb-[-1px] select-none">Like</span>
        </PostRendererButton>
      {:else}
        {@const { icon, label, colors } = reactionRender[post.user_reaction]}
        <PostRendererButton customColors={colors} as="summary" aria-describedby="post-{post.id}-reaction-bar">
          <svelte:component this={icon} animatable={false} {...rendererButtonAttributes} />
          <span class="mb-[-1px] select-none text-black dark:text-white">{label}</span>
        </PostRendererButton>
      {/if}
      <ReactionBar
        id="post-{post.id}-reaction-bar"
        class="animate-fly absolute bottom-full -translate-y-1"
        style="--fly-translate-y:1rem"
        currentReaction={post.user_reaction}
        forId={post.id}
        forType="post"
        updateReaction={(reaction) => {
          updateReaction(reaction);
          if (reactionBar) {
            reactionBar.open = false;
          }
        }}
        revertReaction={() => {
          updateReaction(previousReaction);
          previousReaction = null;
        }}
      />
    </details>
    <PostRendererButton as="a" href="/posts/{post.id}#comments">
      <Comment {...rendererButtonAttributes} />
      <span class="mb-[-1px]">Comment</span>
    </PostRendererButton>
    <PostRendererButton as="div">
      <Share {...rendererButtonAttributes} />
      <span class="mb-[-1px]">Share</span>
    </PostRendererButton>
  </div>
</article>
