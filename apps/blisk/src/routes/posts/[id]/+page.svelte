<script lang="ts">
  import CommentRenderer from "$components/CommentRenderer.svelte";
  import CommentForm from "$components/CommentForm.svelte";
  import MarkdownRenderer from "$components/MarkdownRenderer.svelte";
  import { reactionRender } from "$components/renderer-constants.js";
  import type { Comment, ReactionType, Ref } from "$lib/types.js";
  import ThumbUp from "$components/icons/ThumbUp.svelte";
  import ReactionBar from "$components/ReactionBar.svelte";
  import CommentIcon from "$components/icons/Comment.svelte";
  import Share from "$components/icons/Share.svelte";
  import CommentRendererButton from "$components/CommentRendererButton.svelte";
  import ThumbUpFilled from "$components/icons/ThumbUpFilled.svelte";
  import VirtualScroller from "$components/VirtualScroller.svelte";
  import { rendererButtonAttributes } from "$components/renderer-constants.js";
  import { OPTIMISTIC_ID } from "$lib/constants.js";
  import { fetchBackend } from "$lib/backend.client.js";

  const { data } = $props();

  const post = $derived.by(() => {
    const state = $state(data.post);
    return state;
  });
  let previousReaction: ReactionType | null = null;
  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const updateReaction = (reaction: ReactionType | null) => {
    previousReaction = post.user_reaction;
    post.user_reaction = reaction;
  };
</script>

<article class="flex h-full w-full max-w-6xl flex-col gap-8 p-2 md:py-8">
  <div class="flex flex-col gap-8">
    <h1 class="-order-1">{post.title}</h1>
    <div class="-order-2 flex flex-row flex-wrap items-center gap-4 font-semibold leading-10 tracking-tight">
      <img src="/no-avatar.webp" class="border-border-light dark:border-border-dark size-12 select-none rounded-full border shadow-lg" alt="" />
      <div>
        <div class="text-comment flex flex-row flex-wrap items-center gap-1 text-base">
          <a href="/users/{post.author_name}" class="link sm -mt-[1px]">{post.author_name}</a>
          <span>•</span>
          <div>Just now</div>
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
      <details bind:this={reactionBar} class="relative">
        {#if !post.user_reaction}
          <CommentRendererButton as="summary" aria-describedby="reaction-bar-{post.id}">
            <ThumbUp {...rendererButtonAttributes} /> <span class="mb-[-1px] pr-1">Like</span>
          </CommentRendererButton>
        {:else}
          {@const { icon, label, colors } = reactionRender[post.user_reaction]}
          <CommentRendererButton customColors={colors} as="summary" aria-describedby="reaction-bar-{post.id}">
            <svelte:component this={icon} animatable={false} {...rendererButtonAttributes} />
            <span class="mb-[-1px] pr-1 text-black dark:text-white">{label}</span>
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
      <CommentRendererButton as="a" href="#comments">
        <CommentIcon {...rendererButtonAttributes} />
        <span class="mb-[-1px] pr-1">Comment</span>
      </CommentRendererButton>
      <CommentRendererButton as="div">
        <Share {...rendererButtonAttributes} />
        <span class="mb-[-1px] pr-1">Share</span>
      </CommentRendererButton>
    </div>
  </div>
  <section id="comments" class="flex h-full flex-col gap-3">
    <h2 class="sr-only">Comments</h2>
    <CommentForm
      parentId={null}
      updateReplies={(newComment) => post.comments.unshift(newComment)}
      revertReplies={() => (post.comments = post.comments.filter((comment) => comment.id !== OPTIMISTIC_ID))}
    />
    <VirtualScroller
      bind:items={post.comments}
      loadMore={async () => {
        const data = await fetchBackend<Comment[]>(`/comments?post_id=${post.id}&offset=${post.comments.length}`, {
          signal: AbortSignal.timeout(10000),
        });
        if (!data.ok) {
          throw new Error(data.error);
        }
        return data.data;
      }}
    >
      {#snippet renderer(comment: Ref<Comment>)}
        <div class="pb-3">
          <!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
          <!-- svelte-ignore binding_property_non_reactive -->
          <CommentRenderer bind:comment={comment.ref} currentUser={data.user?.name} />
        </div>
      {/snippet}
    </VirtualScroller>
  </section>
</article>
