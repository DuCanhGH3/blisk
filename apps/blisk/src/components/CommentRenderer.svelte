<script lang="ts">
  // Component is (mostly) stateless so that it works with the virtual scroller.
  import CommentRendererButton from "$components/CommentRendererButton.svelte";
  import CommentIcon from "$components/icons/Comment.svelte";
  import Share from "$components/icons/Share.svelte";
  import ThumbUp from "$components/icons/ThumbUp.svelte";
  import ReactionBar from "$components/ReactionBar.svelte";
  import { clsx } from "$lib/clsx";
  import type { Comment, ReactionType } from "$lib/types";
  import CommentForm from "./CommentForm.svelte";
  import { rendererButtonAttributes, reactionRender } from "./renderer-constants";
  import MarkdownRenderer from "./MarkdownRenderer.svelte";

  interface CommentProps {
    /**
     * The comment to be rendered. Must be a state for the component to work
     * properly.
     */
    comment: Comment;
    /**
     * Called whenever the user's reaction to a comment is updated. Used for
     * updating said comment's `user_reaction` state.
     * @param reaction
     */
    updateReaction(comment: Comment, reaction: ReactionType | null): void;
    /**
     * Called whenever a reply is added to a comment. Used for updating said comment's
     * `replies` state.
     * @param reaction
     */
    updateReplies(comment: Comment, newReply: Comment): void;
  }

  const { comment, updateReaction: updateReactionState, updateReplies }: CommentProps = $props();

  let previousReaction: ReactionType | null = null;

  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const updateReaction = (reaction: ReactionType | null) => {
    previousReaction = comment.user_reaction;
    updateReactionState(comment, reaction);
  };
</script>

<article>
  <h3 class="sr-only">A comment from user {comment.author_name}</h3>
  <div class="box flex flex-col gap-2 rounded-[21px] p-2.5 shadow-md">
    <div class="flex flex-row items-center gap-2">
      <img src="/no-avatar.webp" class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg" alt="" />
      <span class="flex flex-col gap-1">
        <a href="/users/{comment.author_name}" class="link sm text-sm">{comment.author_name}</a>
        <span class="text-comment text-xs">Just now</span>
      </span>
    </div>
    <MarkdownRenderer source={comment.content} startingHeading={4} />
    <div class="-m-1 mt-0 flex w-fit flex-row flex-wrap gap-2">
      <details bind:this={reactionBar} class="relative">
        {#if !comment.user_reaction}
          <CommentRendererButton as="summary" aria-describedby="reaction-bar-{comment.id}">
            <ThumbUp {...rendererButtonAttributes} /> Like
          </CommentRendererButton>
        {:else}
          {@const { icon, label, colors } = reactionRender[comment.user_reaction]}
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
      <CommentForm
        parentId={comment.id}
        updateReplies={(reply) => {
          updateReplies(comment, reply);
        }}
      />
    </div>
    {#if comment.children && comment.children.length > 0}
      <ul class="flex flex-col gap-3 pt-3">
        {#each comment.children as reply (reply.id)}
          <li>
            <svelte:self comment={reply} updateReaction={updateReactionState} {updateReplies} />
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</article>
