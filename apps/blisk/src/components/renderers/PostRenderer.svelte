<script lang="ts">
  // Component is (mostly) stateless so that it works with the virtual scroller.
  import { page } from "$app/stores";
  import type { Post, ReactionType } from "$lib/types";
  import { getImage, getLoginUrl, getProfilePicture, getTopReactions, updateReactionMetadata } from "$lib/utils";
  import Comment from "../icons/Comment.svelte";
  import Share from "../icons/Share.svelte";
  import ThumbUp from "../icons/ThumbUp.svelte";
  import ThumbUpFilled from "../icons/ThumbUpFilled.svelte";
  import MarkdownRenderer from "./MarkdownRenderer.svelte";
  import PostRendererButton from "./PostRendererButton.svelte";
  import ReactionBar from "./ReactionBar.svelte";
  import { svgIconAttrs, reactionRender } from "./renderer-constants";
  import TooltipHover from "../TooltipHover.svelte";
  import { INITIAL_REACTION_METADATA } from "$lib/constants";

  interface PostRendererProps {
    /**
     * The post to be rendered. Must be a state for the component to work properly.
     */
    post: Post;
  }

  const { post = $bindable() }: PostRendererProps = $props();

  let reactionBar = $state<HTMLDetailsElement | null>(null);

  const isLoggedIn = $derived(!!$page.data.user);
  const loginUrl = $derived(getLoginUrl($page.url.pathname));

  let previousReaction: ReactionType | null = null;

  const updateReaction = (reaction: ReactionType | null) => {
    previousReaction = post.user_reaction;
    post.user_reaction = reaction;
    if (!post.reactions) post.reactions = INITIAL_REACTION_METADATA;
    updateReactionMetadata(post.reactions, previousReaction, post.user_reaction);
  };
</script>

<article class="box flex flex-col gap-3 rounded-[31px] p-4 shadow-md">
  <h3 class="h1 -order-1 mb-4">{post.title}</h3>
  <div class="-order-2 flex flex-row flex-wrap gap-2 font-semibold leading-10 tracking-tight">
    <img
      src={getProfilePicture(post.author_picture)}
      class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg"
      width={40}
      height={40}
      alt=""
    />
    <div>
      <div class="text-comment flex flex-row flex-wrap items-center gap-1 text-sm">
        <a href="/users/{post.author_name}" class="link sm">{post.author_name}</a>
        <span>•</span>
        <span class="mb-[-1px]">
          <TooltipHover tooltipId="post-{post.id}-timestamp-tooltip" content="Just now">Just now</TooltipHover>
        </span>
      </div>
      <div class="flex flex-row flex-wrap items-center gap-1 text-base">
        {#snippet showBookTitle(name: string | null, title: string, summary: string)}
          <TooltipHover tooltipId="post-{post.id}-book-tooltip">
            {#snippet content()}
              <div class="flex flex-col gap-4 overflow-y-auto p-2">
                <img src={getImage(post.book_cover, "/test-cover.jpg")} class="h-48 w-32 select-none rounded-[5px]" alt="" width={128} height={192} />
                <div class="flex flex-col gap-2">
                  <span class="h3">{title}</span>
                  <MarkdownRenderer source={summary} startingHeading={7} />
                </div>
              </div>
            {/snippet}
            {#if name}
              <a href="/books/{name}" class="link no-color sm"><span class="sr-only">Book: </span><i>{title}</i></a>
            {:else}
              <i><span class="sr-only">Book: </span>{title}</i>
            {/if}
          </TooltipHover>
        {/snippet}
        {#if post.book_reaction === "like"}
          <ThumbUpFilled
            width={20}
            height={20}
            class="fill-accent-light dark:fill-accent-dark hidden h-auto w-5 md:block"
            aria-hidden
            tabindex={-1}
          />
          <span class="text-accent-light dark:text-accent-dark">
            {#if post.book_title && post.book_synopsis}
              Recommends {@render showBookTitle(post.book_name, post.book_title, post.book_synopsis)}
            {:else}
              Recommended
            {/if}
          </span>
        {:else}
          <ThumbUpFilled
            width={20}
            height={20}
            class="fill-error-light dark:fill-error-dark hidden h-auto w-5 -scale-y-100 md:block"
            aria-hidden
            tabindex={-1}
          />
          <span class="text-error-light dark:text-error-dark">
            {#if post.book_title && post.book_synopsis}
              Does not recommend {@render showBookTitle(post.book_name, post.book_title, post.book_synopsis)}
            {:else}
              Not recommended
            {/if}
          </span>
        {/if}
      </div>
    </div>
  </div>
  <MarkdownRenderer source={post.content} startingHeading={4} />
  {#if post.reactions && post.reactions.total > 0}
    <div class="flex flex-row items-center gap-1" interactive={false}>
      <!-- TODO: move top_reactions to client -->
      {#each getTopReactions(post.reactions) as reaction}
        {@const mappedRender = reactionRender[reaction]}
        {@const Icon = mappedRender.icon}
        <Icon {...svgIconAttrs} animatable={false} />
      {/each}
      <span class="px-1">
        {post.reactions.total}<span class="sr-only"> reaction{post.reactions.total === 1 ? "" : "s"}</span>
      </span>
    </div>
  {/if}
  <div class="order-1 -m-1 flex flex-row flex-wrap gap-3">
    {#if isLoggedIn}
      <details bind:this={reactionBar} class="relative flex-grow">
        {#if !post.user_reaction}
          <PostRendererButton as="summary" aria-describedby="post-{post.id}-reaction-bar">
            <ThumbUp {...svgIconAttrs} />
            <span class="mb-[-1px] select-none">Like</span>
          </PostRendererButton>
        {:else}
          {@const { icon: Icon, label, colors } = reactionRender[post.user_reaction]}
          <PostRendererButton customColors={colors} as="summary" aria-describedby="post-{post.id}-reaction-bar">
            <Icon animatable={false} {...svgIconAttrs} />
            <span class="mb-[-1px] select-none">{label}</span>
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
    {:else}
      <PostRendererButton as="a" href={loginUrl}>
        <ThumbUp {...svgIconAttrs} />
        <span class="mb-[-1px] select-none">Like</span>
      </PostRendererButton>
    {/if}
    <PostRendererButton as="a" href="/posts/{post.id}#comments">
      <Comment {...svgIconAttrs} />
      <span class="mb-[-1px]">Comment</span>
    </PostRendererButton>
    <PostRendererButton as="div">
      <Share {...svgIconAttrs} />
      <span class="mb-[-1px]">Share</span>
    </PostRendererButton>
  </div>
</article>
