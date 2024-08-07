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
  import ThreeDots from "./icons/ThreeDots.svelte";
  import Menu from "./Menu.svelte";
  import MenuItem from "./MenuItem.svelte";
  import Pencil from "./icons/Pencil.svelte";
  import CommentEditor from "./CommentEditor.svelte";

  interface CommentProps {
    /**
     * The comment to be rendered. Must be a state for the component to work properly.
     */
    comment: Comment;
  }

  let { comment = $bindable() }: CommentProps = $props();

  let previousReaction: ReactionType | null = null;

  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const toggleEditingMode = () => {
    comment.is_editing = !comment.is_editing;
  };

  const updateReaction = (reaction: ReactionType | null) => {
    previousReaction = comment.user_reaction;
    comment.user_reaction = reaction;
  };

  const updateReplies = (reply: Comment) => {
    if (!comment.children) comment.children = [];
    comment.children.unshift(reply);
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
    {#if !comment.is_editing}
      <MarkdownRenderer source={comment.content} startingHeading={4} />
      <div class="-m-1 mt-0 flex w-fit flex-row flex-wrap gap-2">
        <details bind:this={reactionBar} class="relative">
          {#if !comment.user_reaction}
            <CommentRendererButton as="summary" aria-describedby="reaction-bar-{comment.id}">
              <ThumbUp {...rendererButtonAttributes} /> <span class="pr-1">Like</span>
            </CommentRendererButton>
          {:else}
            {@const { icon, label, colors } = reactionRender[comment.user_reaction]}
            <CommentRendererButton customColors={colors} as="summary" aria-describedby="reaction-bar-{comment.id}">
              <svelte:component this={icon} animatable={false} {...rendererButtonAttributes} />
              <span class="pr-1 text-black dark:text-white">{label}</span>
            </CommentRendererButton>
          {/if}
          <ReactionBar
            id="reaction-bar-{comment.id}"
            class="animate-fly absolute bottom-full -translate-y-1"
            style="--fly-translate-y:1rem"
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
          <CommentIcon {...rendererButtonAttributes} /> <span class="pr-1">Comment</span>
        </CommentRendererButton>
        <CommentRendererButton as="div">
          <Share {...rendererButtonAttributes} /> <span class="pr-1">Share</span>
        </CommentRendererButton>
        <details class="relative">
          <CommentRendererButton as="summary" aria-describedby="menu-bar-{comment.id}">
            <ThreeDots {...rendererButtonAttributes} /> <span class="sr-only">More</span>
          </CommentRendererButton>
          <Menu
            id="menu-bar-{comment.id}"
            class="animate-fly dark:bg-neutral-915 bottom-full z-10 w-32 -translate-y-1 bg-white"
            style="--fly-translate-y:1rem"
          >
            <div>
              <MenuItem as="button" onclick={toggleEditingMode}>
                <Pencil width={20} height={20} class="mr-2 h-auto w-5" aria-hidden="true" tabindex={-1} /> Edit
              </MenuItem>
            </div>
          </Menu>
        </details>
      </div>
    {:else}
      <CommentEditor bind:comment={comment} />
    {/if}
  </div>
  <div
    class={clsx(
      "before:bg-neutral-250 relative ml-3 pl-3 before:absolute before:inset-y-0",
      "before:left-0 before:w-px before:content-[''] dark:before:bg-neutral-800"
    )}
  >
    <input class="peer sr-only" type="checkbox" checked={false} id="comment-toggle-{comment.id}" />
    <div class="hidden pt-3 peer-checked:block">
      <CommentForm parentId={comment.id} {updateReplies} />
    </div>
    {#if comment.children && comment.children.length > 0}
      <ul class="flex flex-col gap-3 pt-3">
        {#each comment.children as reply (reply.id)}
          <li>
            <svelte:self comment={reply} />
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</article>
