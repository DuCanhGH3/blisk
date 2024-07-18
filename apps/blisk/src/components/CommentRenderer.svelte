<script lang="ts">
  import CommentRendererButton from "$components/CommentRendererButton.svelte";
  import CommentIcon from "$components/icons/Comment.svelte";
  import Share from "$components/icons/Share.svelte";
  import ThumbUp from "$components/icons/ThumbUp.svelte";
  import ReactionBar from "$components/ReactionBar.svelte";
  import { clsx } from "$lib/clsx";
  import type { Comment, ReactionType } from "$lib/types";
  import CommentForm from "./CommentForm.svelte";
  import type { IconProps } from "./icons/types";
  import { reactionRender } from "./renderer-constants";

  interface CommentProps {
    comment: Comment;
    username: string | undefined;
  }

  const { comment, username }: CommentProps = $props();

  let currentReaction = $state<ReactionType | null>(null);

  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const rendererButtonAttributes = {
    width: 24,
    height: 24,
    class: "h-6 w-auto",
    "aria-hidden": "true",
    tabindex: -1,
  } satisfies IconProps;
</script>

<div class="box flex flex-col gap-2 rounded-[21px] p-2.5 shadow-md">
  <div class="flex flex-row items-center gap-2">
    <img src="/no-avatar.webp" class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg" alt="" />
    <span class="flex flex-col gap-1">
      <a href="/users/{comment.author_name}" class="link sm text-sm">{comment.author_name}</a>
      <span class="text-comment text-xs">Just now</span>
    </span>
  </div>
  <div>{comment.content}</div>
  <div class="-m-1 mt-0 flex w-fit flex-row gap-2">
    <details bind:this={reactionBar} class="relative">
      {#if currentReaction === null}
        <CommentRendererButton as="summary" aria-describedby="reaction-bar-{comment.id}">
          <ThumbUp {...rendererButtonAttributes} /> Like
        </CommentRendererButton>
      {:else}
        {@const { icon, label, colors } = reactionRender[currentReaction]}
        <CommentRendererButton customColors={colors} as="summary" aria-describedby="reaction-bar-{comment.id}">
          <svelte:component this={icon} animatable={false} {...rendererButtonAttributes} />
          <span class="text-black dark:text-white">{label}</span>
        </CommentRendererButton>
      {/if}
      <ReactionBar
        id="reaction-bar-{comment.id}"
        class="animate-fly absolute top-0 translate-y-[calc(-100%-4px)]"
        style="--fly-translate:0.25rem"
        forId={comment.id}
        forType="comment"
        updateReaction={(reaction) => {
          currentReaction = reaction;
          if (reactionBar) {
            reactionBar.open = false;
          }
        }}
        revertReaction={() => (currentReaction = null)}
      />
    </details>
    <CommentRendererButton as="label" id="comment-toggle-label-{comment.id}" for="comment-toggle-{comment.id}">
      <CommentIcon {...rendererButtonAttributes} /> Comment
    </CommentRendererButton>
    <CommentRendererButton as="div">
      <Share {...rendererButtonAttributes} /> Share
    </CommentRendererButton>
  </div>
</div>
<div
  class={clsx(
    "before:bg-neutral-250 relative ml-3 pl-3 before:absolute before:inset-y-0",
    "before:left-0 before:w-px before:content-[''] dark:before:bg-neutral-800"
  )}
>
  <input class="peer sr-only" type="checkbox" checked={false} id="comment-toggle-{comment.id}" />
  <div class="hidden pt-3 peer-checked:block">
    <CommentForm parentId={comment.id} updateComments={(newComment) => comment.replies.unshift(newComment)} />
  </div>
  {#if comment.replies && comment.replies.length > 0}
    <ul class="flex flex-col gap-3 pt-3">
      {#each comment.replies as reply}
        <li>
          <svelte:self comment={reply} {username} />
        </li>
      {/each}
    </ul>
  {/if}
</div>
