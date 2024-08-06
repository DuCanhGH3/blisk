<script lang="ts">
  import CommentRenderer from "$components/CommentRenderer.svelte";
  import CommentForm from "$components/CommentForm.svelte";
  import MarkdownRenderer from "$components/MarkdownRenderer.svelte";
  import { reactionRender } from "$components/renderer-constants.js";
  import type { Comment, ReactionType } from "$lib/types.js";
  import type { IconProps } from "$components/icons/types.js";
  import ThumbUp from "$components/icons/ThumbUp.svelte";
  import ReactionBar from "$components/ReactionBar.svelte";
  import CommentIcon from "$components/icons/Comment.svelte";
  import Share from "$components/icons/Share.svelte";
  import CommentRendererButton from "$components/CommentRendererButton.svelte";
  import ThumbUpFilled from "$components/icons/ThumbUpFilled.svelte";
  import VirtualScroller from "$components/VirtualScroller.svelte";

  const { data } = $props();

  let comments = $state(data.comments);
  let previousReaction: ReactionType | null = null;
  let currentReaction = $state<ReactionType | null>(data.post.user_reaction);
  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const rendererButtonAttributes = {
    width: 24,
    height: 24,
    class: "h-6 w-auto",
    "aria-hidden": "true",
    tabindex: -1,
  } satisfies IconProps;
</script>

<article class="flex h-full w-full max-w-6xl flex-col gap-8 p-2 md:py-8">
  <div class="flex flex-col gap-8">
    <h1 class="-order-1">{data.post.title}</h1>
    <div class="-order-2 flex flex-row flex-wrap items-center gap-4 font-semibold leading-10 tracking-tight">
      <img src="/no-avatar.webp" class="border-border-light dark:border-border-dark size-12 select-none rounded-full border shadow-lg" alt="" />
      <div>
        <div class="text-comment flex flex-row flex-wrap items-center gap-1 text-base">
          <a href="/users/{data.post.author_name}" class="link sm -mt-[1px]">{data.post.author_name}</a>
          <span>•</span>
          <div>Just now</div>
        </div>
        <div class="flex flex-row flex-wrap gap-1 text-lg">
          {#if data.post.reaction === "like"}
            <ThumbUpFilled width={24} height={24} class="fill-accent-light dark:fill-accent-dark h-auto w-6" aria-hidden tabindex={-1} />
            <span class="text-accent-light dark:text-accent-dark">Recommended</span>
          {:else}
            <ThumbUpFilled width={24} height={24} class="fill-error-light dark:fill-error-dark h-auto w-6 -scale-y-100" aria-hidden tabindex={-1} />
            <span class="text-error-light dark:text-error-dark">Not recommended</span>
          {/if}
        </div>
      </div>
    </div>
    <MarkdownRenderer source={data.post.content} startingHeading={2} />
    <div class="order-1 -m-1 mt-0 flex w-fit flex-row flex-wrap gap-2">
      <details bind:this={reactionBar} class="relative">
        {#if !currentReaction}
          <CommentRendererButton as="summary" aria-describedby="reaction-bar-{data.post.id}">
            <ThumbUp {...rendererButtonAttributes} /> <span class="mb-[-1px]">Like</span>
          </CommentRendererButton>
        {:else}
          {@const { icon, label, colors } = reactionRender[currentReaction]}
          <CommentRendererButton customColors={colors} as="summary" aria-describedby="reaction-bar-{data.post.id}">
            <svelte:component this={icon} animatable={false} {...rendererButtonAttributes} />
            <span class="mb-[-1px] text-black dark:text-white">{label}</span>
          </CommentRendererButton>
        {/if}
        <ReactionBar
          id="reaction-bar-{data.post.id}"
          class="animate-fly absolute top-0 translate-y-[calc(-100%-4px)]"
          style="--fly-translate:0.25rem"
          forId={data.post.id}
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
      <CommentRendererButton as="a" href="#comments">
        <CommentIcon {...rendererButtonAttributes} />
        <span class="mb-[-1px]">Comment</span>
      </CommentRendererButton>
      <CommentRendererButton as="div">
        <Share {...rendererButtonAttributes} />
        <span class="mb-[-1px]">Share</span>
      </CommentRendererButton>
    </div>
  </div>
  <section id="comments" class="flex h-full flex-col gap-3">
    <h2 class="sr-only">Comments</h2>
    <CommentForm parentId={null} updateReplies={(newComment) => comments.unshift(newComment)} />
    {#snippet renderer(comment: Comment)}
      <div class="pb-3">
        <CommentRenderer
          {comment}
          updateReaction={(comment, reaction) => {
            comment.user_reaction = reaction;
          }}
          updateReplies={(comment, reply) => {
            if (!comment.children) comment.children = [];
            comment.children.unshift(reply);
          }}
        />
      </div>
    {/snippet}
    <VirtualScroller items={comments} {renderer} />
  </section>
</article>
