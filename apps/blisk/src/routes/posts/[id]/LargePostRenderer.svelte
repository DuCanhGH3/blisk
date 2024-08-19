<script lang="ts">
  import { page } from "$app/stores";
  import { OPTIMISTIC_ID } from "$lib/constants.js";
  import type { Comment, Post, ReactionType, Ref, RequireFields } from "$lib/types.js";
  import { getLoginUrl, getProfilePicture } from "$lib/utils.js";
  import ThumbUp from "$components/icons/ThumbUp.svelte";
  import CommentForm from "$components/renderers/CommentForm.svelte";
  import MarkdownRenderer from "$components/renderers/MarkdownRenderer.svelte";
  import CommentRenderer from "$components/renderers/CommentRenderer.svelte";
  import CommentRendererButton from "$components/renderers/CommentRendererButton.svelte";
  import ReactionBar from "$components/renderers/ReactionBar.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
  import { reactionRender, svgIconAttrs } from "$components/renderers/renderer-constants.js";
  import CommentIcon from "$components/icons/Comment.svelte";
  import Share from "$components/icons/Share.svelte";
  import ThumbUpFilled from "$components/icons/ThumbUpFilled.svelte";
  import TooltipHover from "$components/TooltipHover.svelte";

  interface LargePostRendererProps {
    /**
     * The post to be renderered.
     */
    post: RequireFields<Post, "comments">;
    /**
     * Whether the comment form should be shown.
     */
    showCommentForm?: boolean;
    /**
     * A function that should load more comments.
     */
    loadMoreComments?(): Promise<Comment[]> | Comment[];
  }

  let { post = $bindable(), showCommentForm = true, loadMoreComments }: LargePostRendererProps = $props();

  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const isLoggedIn = $derived(!!$page.data.user);
  const loginUrl = $derived(getLoginUrl($page.url.pathname));

  let previousReaction: ReactionType | null = null;

  const updateReaction = (reaction: ReactionType | null) => {
    previousReaction = post.user_reaction;
    post.user_reaction = reaction;
  };
</script>

<article class="flex h-full w-full max-w-6xl flex-col gap-8 p-2 md:py-8">
  <div class="flex flex-col gap-8">
    <h1 class="-order-1">{post.title}</h1>
    <div class="-order-2 flex flex-row flex-wrap items-center gap-4 font-semibold leading-10 tracking-tight">
      <img
        src={getProfilePicture(post.author_picture)}
        class="border-border-light dark:border-border-dark size-12 select-none rounded-full border shadow-lg"
        alt=""
      />
      <div>
        <div class="text-comment flex flex-row flex-wrap items-center gap-1 text-base">
          <a href="/users/{post.author_name}" class="link sm -mt-[1px]">{post.author_name}</a>
          <span>•</span>
          <TooltipHover tooltipId="post-{post.id}-timestamp-tooltip" content="Just now">Just now</TooltipHover>
        </div>
        <div class="flex flex-row flex-wrap gap-1 text-lg">
          {#if post.reaction === "like"}
            <ThumbUpFilled width={24} height={24} class="fill-accent-light dark:fill-accent-dark h-auto w-6" aria-hidden tabindex={-1} />
            <span class="text-accent-light dark:text-accent-dark">Recommended</span>
          {:else}
            <ThumbUpFilled width={24} height={24} class="fill-error-light dark:fill-error-dark h-auto w-6 -scale-y-100" aria-hidden tabindex={-1} />
            <span class="text-error-light dark:text-error-dark">Not recommended</span>
          {/if}
        </div>
      </div>
    </div>
    <MarkdownRenderer source={post.content} startingHeading={2} />
    <div class="order-1 -m-1 mt-0 flex w-fit flex-row flex-wrap gap-2">
      {#if isLoggedIn}
        <details bind:this={reactionBar} class="relative">
          {#if !post.user_reaction}
            <CommentRendererButton as="summary" aria-describedby="reaction-bar-{post.id}">
              <ThumbUp {...svgIconAttrs} /> <span class="mb-[-1px] pr-1">Like</span>
            </CommentRendererButton>
          {:else}
            {@const { icon: Icon, label, colors } = reactionRender[post.user_reaction]}
            <CommentRendererButton customColors={colors} as="summary" aria-describedby="reaction-bar-{post.id}">
              <Icon animatable={false} {...svgIconAttrs} />
              <span class="mb-[-1px] pr-1 text-wood-900 dark:text-white">{label}</span>
            </CommentRendererButton>
          {/if}
          <ReactionBar
            id="reaction-bar-{post.id}"
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
      {:else}
        <CommentRendererButton as="a" href={loginUrl}>
          <ThumbUp {...svgIconAttrs} />
          <span class="mb-[-1px] pr-1">Like</span>
        </CommentRendererButton>
      {/if}
      <CommentRendererButton as="a" href="#comments">
        <CommentIcon {...svgIconAttrs} />
        <span class="mb-[-1px] pr-1">Comment</span>
      </CommentRendererButton>
      <CommentRendererButton as="div">
        <Share {...svgIconAttrs} />
        <span class="mb-[-1px] pr-1">Share</span>
      </CommentRendererButton>
    </div>
  </div>
  <section id="comments" class="flex h-full flex-col gap-3">
    <h2 class="sr-only">Comments</h2>
    {#if showCommentForm && isLoggedIn}
      <!-- TODO(ducanhgh): VirtualScroller's update doesn't seem to be triggered when we push a new entry -->
      <!-- to `post.comments`, so we reassign `post.comments` for now. Hopefully this is just a bug. -->
      <CommentForm
        parentId={null}
        updateReplies={(newComment) => (post.comments = [newComment, ...(post.comments ?? [])])}
        revertReplies={() => (post.comments = post.comments.filter((comment) => comment.id !== OPTIMISTIC_ID))}
      />
    {/if}
    <VirtualScroller bind:items={post.comments} loadMore={loadMoreComments}>
      {#snippet renderer(comment: Ref<Comment>)}
        <div class="pb-3">
          <!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
          <!-- svelte-ignore binding_property_non_reactive -->
          <CommentRenderer
            bind:comment={comment.ref}
            depth={0}
            removeComment={(commentToFilter) => {
              const oldComments = [...post.comments];
              post.comments = post.comments.filter((comment) => comment.id !== commentToFilter.id);
              return () => {
                post.comments = oldComments;
              };
            }}
          />
        </div>
      {/snippet}
    </VirtualScroller>
  </section>
</article>
