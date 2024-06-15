<script lang="ts">
  import { enhance } from "$app/forms";
  import { page } from "$app/stores";
  import Input from "$components/Input.svelte";
  import { OPTIMISTIC_ID } from "$lib/constants.js";
  import Comment from "./Comment.svelte";

  const { data, form } = $props();

  let isCommenting = $state(false);
</script>

<div class="flex w-full max-w-6xl flex-col gap-3 p-2 md:py-8">
  <article class="flex flex-col gap-3">
    <h1 class="order-2">{data.post.title}</h1>
    <h2 class="text-comment order-1 text-xl">{data.post.author_name}</h2>
    <div class="order-3 mt-4">
      {data.post.content}
    </div>
  </article>
  <form
    method="POST"
    action="?/comment"
    class="flex flex-col gap-2"
    use:enhance={({ formData }) => {
      isCommenting = true;
      const content = formData.get("content");
      const author_name = data.user?.name;
      const post_id = Number.parseInt($page.params.id);
      if (typeof author_name === "string" && typeof content === "string" && !Number.isNaN(post_id)) {
        data.comments.unshift({
          id: OPTIMISTIC_ID,
          path: "Top",
          content,
          author_name,
          level: 1,
          post_id,
          replies: [],
        });
      }
      return async ({ update }) => {
        await update();
        isCommenting = false;
      };
    }}
  >
    <Input
      id="comment-content-input"
      label="Content"
      name="content"
      type="text"
      errorText={form?.validationError?.content}
      errorTextId="comment-content-error-text"
    />
    <div>
      <button class="button" disabled={isCommenting}>Comment</button>
    </div>
  </form>
  {#each data.comments as comment}
    <Comment {comment} parentId={null} username={data.user?.name} />
  {/each}
</div>
