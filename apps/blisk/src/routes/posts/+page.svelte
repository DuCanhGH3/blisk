<script lang="ts">
  import { page } from "$app/stores";
  import PostRenderer from "$components/renderers/PostRenderer.svelte";
  import VirtualScroller from "$components/renderers/VirtualScroller.svelte";
  import { fetchBackend } from "$lib/backend.client.js";
  import { clsx } from "$lib/clsx";
  import type { Post, Ref } from "$lib/types";
  import { getProfilePicture } from "$lib/utils";

  const { data } = $props();

  let posts = $state(data.posts);

  $effect(() => {
    posts = data.posts;
  });
</script>

<div class="mx-auto flex w-full max-w-screen-md flex-col gap-8 p-4 md:p-10">
  {#if $page.data.user}
    <div class="box flex flex-row gap-2 rounded-[29px] p-2.5 shadow-md">
      <a href="/users/{$page.data.user.name}">
        <img
          src={getProfilePicture($page.data.user.picture ?? null)}
          class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg"
          width={40}
          height={40}
          alt="Your profile"
        />
      </a>
      <a
        href="/posts/create?redirectTo={$page.url.pathname}"
        class={clsx(
          "text-comment flex h-10 w-full items-center rounded-[20px] border px-2.5 transition-colors duration-100",
          "dark:bg-neutral-915 border-border-light dark:border-border-dark bg-white",
          "hover:bg-neutral-100 hover:dark:bg-neutral-800"
        )}
      >
        Reviewing a book?
      </a>
    </div>
  {/if}
  <section id="posts">
    <VirtualScroller
      bind:items={posts}
      loadMore={async () => {
        if (posts.length === 0) return [];
        const previousLast = posts[posts.length - 1];
        const data = await fetchBackend<Post[]>(`/posts?previous_last=${previousLast.id}`, {
          signal: AbortSignal.timeout(10000),
        });
        if (!data.ok) {
          throw new Error(data.error);
        }
        return data.data;
      }}
    >
      {#snippet renderer(post: Ref<Post>)}
        <div class="pb-4">
          <!-- eslint-disable-next-line svelte/no-unused-svelte-ignore -->
          <!-- svelte-ignore binding_property_non_reactive -->
          <PostRenderer bind:post={post.ref} />
        </div>
      {/snippet}
    </VirtualScroller>
  </section>
</div>
<!-- {#if $page.state.createPost}
  <div class="fixed left-0 top-0 z-50 flex h-full w-full items-center justify-center bg-black/80">
    <CreateBox />
  </div>
{/if} -->
