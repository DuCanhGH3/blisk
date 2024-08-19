<script lang="ts">
  // Component is (mostly) stateless so that it works with the virtual scroller.
  import { page } from "$app/stores";
  import { clsx } from "$lib/clsx";
  import type { Comment, ReactionType } from "$lib/types";
  import { dialog } from "$lib/stores/dialog.svelte";
  import { hotkeys } from "$lib/hotkeys.svelte";
  import { BASE_COMMENTS_LENGTH, OPTIMISTIC_ID } from "$lib/constants";
  import { fetchBackend } from "$lib/backend.client";
  import CommentForm from "./CommentForm.svelte";
  import { svgIconAttrs, reactionRender } from "./renderer-constants";
  import MarkdownRenderer from "./MarkdownRenderer.svelte";
  import ReactionBar from "./ReactionBar.svelte";
  import CommentEditor from "./CommentEditor.svelte";
  import CommentRendererButton from "./CommentRendererButton.svelte";
  import Menu from "../Menu.svelte";
  import MenuItem from "../MenuItem.svelte";
  import ThreeDots from "../icons/ThreeDots.svelte";
  import Pencil from "../icons/Pencil.svelte";
  import Trash from "../icons/Trash.svelte";
  import TooltipHover from "../TooltipHover.svelte";
  import CommentIcon from "../icons/Comment.svelte";
  import Share from "../icons/Share.svelte";
  import ThumbUp from "../icons/ThumbUp.svelte";
  import { getLoginUrl, getProfilePicture, getTopReactions, updateReactionMetadata } from "$lib/utils";

  interface CommentProps {
    /**
     * The comment to be rendered. Must be a state for the component to work properly.
     */
    comment: Comment;
    /**
     * The current comment's render depth.
     */
    depth: number;
    /**
     * The function used to optimistically remove the comment. This should return
     * a function that will be used to revert the optimistic update.
     * @param comment
     */
    removeComment(comment: Comment): () => void;
  }

  let { comment = $bindable(), depth, removeComment }: CommentProps = $props();

  let previousReaction: ReactionType | null = null;
  let deleteCommentError = $state<string | null>(null);
  let reactionBar = $state<HTMLDetailsElement | null>(null);
  let menu = $state<HTMLDetailsElement | null>(null);
  let noMoreReplies = $state(false);

  const isLoggedIn = $derived(!!$page.data.user);
  const loginUrl = $derived(getLoginUrl($page.url.pathname));

  hotkeys([
    [
      "Escape",
      () => {
        if (menu) menu.open = false;
      },
    ],
  ]);

  const loadMoreReplies = async () => {
    // If the number of replies in the object is not higher than `BASE_COMMENTS_LENGTH`, simply assume
    // there's no more reply to be fetched.
    if (noMoreReplies || !comment.children || comment.children.length < BASE_COMMENTS_LENGTH) return;
    const lastSeen = comment.children[comment.children.length - 1];
    const data = await fetchBackend<Comment[]>(`/comments/replies?comment_id=${comment.id}&previous_last=${lastSeen.id}`);
    if (!data.ok) {
      return;
    }
    if (data.data.length === 0) {
      noMoreReplies = true;
      return;
    }
    comment.children.push(...data.data);
  };

  const updateReaction = (reaction: ReactionType | null) => {
    previousReaction = comment.user_reaction;
    comment.user_reaction = reaction;
    updateReactionMetadata(comment.reactions, previousReaction, comment.user_reaction);
  };

  const openDeleteModal = () => {
    dialog.state = {
      type: "action",
      title: "Are you sure you want to delete this?",
      description: "There's no going back :)",
      closeVariant: "error",
      closeText: "Delete",
      async onClose() {
        dialog.state = null;
        const revertOptimistic = removeComment(comment);
        const res = await fetchBackend(`/comments?id=${comment.id}`, {
          noSuccessContent: true,
          method: "DELETE",
          signal: AbortSignal.timeout(10000),
        });
        if (!res.ok) {
          revertOptimistic();
          deleteCommentError = res.error;
        }
      },
      cancelText: "Cancel",
      onCancel() {
        dialog.state = null;
      },
    };
  };
</script>

<article>
  <h3 class="sr-only">A comment from user {comment.author_name}</h3>
  <div class="box flex flex-col gap-2 rounded-[21px] p-2.5 shadow-md">
    <div class="flex flex-row items-center gap-2">
      <img
        src={getProfilePicture(comment.author_picture)}
        class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg"
        width={40}
        height={40}
        alt=""
      />
      <span class="flex flex-col gap-1">
        <a href="/users/{comment.author_name}" class="link sm text-sm">{comment.author_name}</a>
        <TooltipHover class="text-comment text-xs" tooltipId="comment-{comment.id}-timestamp-tooltip" content="Just now">Just now</TooltipHover>
      </span>
    </div>
    {#if !comment.is_editing}
      <MarkdownRenderer source={comment.content} startingHeading={4} />
      <div class="-m-1 mt-0 flex w-fit flex-row flex-wrap items-center gap-2">
        {#if comment.reactions.total > 0}
          <div class="order-last [&>div]:gap-1">
            <CommentRendererButton as="div" interactive={false}>
              {#each getTopReactions(comment.reactions) as reaction}
                {@const mappedRender = reactionRender[reaction]}
                {@const Icon = mappedRender.icon}
                <Icon {...svgIconAttrs} animatable={false} />
              {/each}
              <span class="px-1">
                {comment.reactions.total} <span class="sr-only">reaction{comment.reactions.total === 1 ? "" : "s"}</span>
              </span>
            </CommentRendererButton>
          </div>
        {/if}
        {#if isLoggedIn}
          <details bind:this={reactionBar} class="relative">
            {#if !comment.user_reaction}
              <CommentRendererButton as="summary" aria-describedby="comment-{comment.id}-reaction-bar">
                <ThumbUp {...svgIconAttrs} /> <span class="select-none pr-1">Like</span>
              </CommentRendererButton>
            {:else}
              {@const { icon: Icon, label, colors } = reactionRender[comment.user_reaction]}
              <CommentRendererButton customColors={colors} as="summary" aria-describedby="comment-{comment.id}-reaction-bar">
                <Icon animatable={false} {...svgIconAttrs} />
                <span class="text-wood-900 select-none pr-1 dark:text-white">{label}</span>
              </CommentRendererButton>
            {/if}
            <ReactionBar
              id="comment-{comment.id}-reaction-bar"
              class="animate-fly absolute bottom-full -translate-y-1"
              style="--fly-translate-y:1rem"
              currentReaction={comment.user_reaction}
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
        {:else}
          <CommentRendererButton as="a" href={loginUrl}>
            <ThumbUp {...svgIconAttrs} />
            <span class="select-none pr-1">Like</span>
          </CommentRendererButton>
        {/if}
        {#if depth >= 4}
          <CommentRendererButton as="a" href="/posts/{comment.post_id}/comments/{comment.id}#comments">
            <CommentIcon {...svgIconAttrs} />
            <span class="pr-1">Comment</span>
          </CommentRendererButton>
        {:else if isLoggedIn}
          <CommentRendererButton as="label" id="comment-toggle-label-{comment.id}" for="comment-toggle-{comment.id}">
            <CommentIcon {...svgIconAttrs} />
            <span class="pr-1">Comment</span>
          </CommentRendererButton>
        {:else}
          <CommentRendererButton as="a" href={loginUrl}>
            <CommentIcon {...svgIconAttrs} />
            <span class="pr-1">Comment</span>
          </CommentRendererButton>
        {/if}
        <CommentRendererButton as="div">
          <Share {...svgIconAttrs} />
          <span class="pr-1">Share</span>
        </CommentRendererButton>
        {#if $page.data.user?.name === comment.author_name}
          <details bind:this={menu} class="relative">
            <CommentRendererButton as="summary" aria-describedby="comment-{comment.id}-menu-bar">
              <ThreeDots {...svgIconAttrs} /> <span class="sr-only">More</span>
            </CommentRendererButton>
            <Menu
              id="comment-{comment.id}-menu-bar"
              class="animate-fly dark:bg-neutral-915 bottom-full z-10 w-32 -translate-y-1 bg-white"
              style="--fly-translate-y:1rem"
            >
              <div>
                <MenuItem
                  as="button"
                  onclick={() => {
                    comment.is_editing = !comment.is_editing;
                  }}
                >
                  <Pencil width={20} height={20} class="mr-2 h-auto w-5" aria-hidden="true" tabindex={-1} /> Edit
                </MenuItem>
                <MenuItem as="button" customColors="text-error-light dark:text-error-dark" onclick={openDeleteModal}>
                  <Trash width={20} height={20} class="mr-2 h-auto w-5" aria-hidden="true" tabindex={-1} /> Delete
                </MenuItem>
              </div>
            </Menu>
          </details>
        {/if}
        {#if deleteCommentError}
          <p class="text-error-light dark:text-error-dark" role="alert">{deleteCommentError}</p>
        {/if}
      </div>
    {:else}
      <CommentEditor bind:comment />
    {/if}
  </div>
  <div
    class={clsx(
      "before:bg-neutral-250 relative ml-3 pl-3 before:absolute before:inset-y-0",
      "before:left-0 before:w-px before:content-[''] dark:before:bg-neutral-800"
    )}
  >
    {#if depth < 4 && isLoggedIn}
      <input class="peer sr-only" type="checkbox" checked={false} id="comment-toggle-{comment.id}" />
      <div class="hidden pt-3 peer-checked:block">
        <CommentForm
          parentId={comment.id}
          updateReplies={(reply: Comment) => {
            comment.children = [reply, ...(comment.children ?? [])];
          }}
          revertReplies={() => {
            if (!comment.children) return;
            comment.children = comment.children.filter((reply) => reply.id !== OPTIMISTIC_ID);
          }}
        />
      </div>
    {/if}
    {#if comment.children && comment.children.length > 0}
      <ul class="flex flex-col gap-3 pt-3">
        {#each comment.children as reply (reply.id)}
          <li>
            <svelte:self comment={reply} depth={depth + 1} {removeComment} />
          </li>
        {/each}
      </ul>
      <!-- If at any point in time, this comment does not have at least `BASE_COMMENTS_LENGTH` replies, -->
      <!-- assume that it does not have any more reply. -->
      {#if comment.children.length >= BASE_COMMENTS_LENGTH && !noMoreReplies}
        <button class="text-comment block py-1.5 text-sm underline" onclick={loadMoreReplies}>View more replies</button>
      {/if}
    {:else if depth >= 4}
      <a class="text-comment block py-1.5 text-sm underline" href="/posts/{comment.post_id}/comments/{comment.id}#comments">View more replies</a>
    {/if}
  </div>
</article>
