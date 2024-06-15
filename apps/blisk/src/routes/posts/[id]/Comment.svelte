<script lang="ts">
  import { enhance } from "$app/forms";
  import { page } from "$app/stores";
  import Input from "$components/Input.svelte";
  import { OPTIMISTIC_ID } from "$lib/constants";
  import type { Comment } from "$lib/types";

  interface CommentProps {
    comment: Comment;
    parentId: number | null;
    username: string | undefined;
  }

  const { comment, parentId, username }: CommentProps = $props();

  let isReplying = $state(false);
</script>

<div>
  <div>{comment.author_name}</div>
  <div>{comment.content}</div>
  <div class="pl-3">
    <form
      method="POST"
      action={`?/comment&parentId=${comment.id}`}
      class="flex flex-col gap-2"
      use:enhance={({ formData }) => {
        isReplying = true;
        const content = formData.get("content");
        const post_id = Number.parseInt($page.params.id);
        if (typeof username === "string" && typeof content === "string" && !Number.isNaN(post_id)) {
          comment.replies.unshift({
            id: OPTIMISTIC_ID,
            path: comment.path,
            content,
            author_name: username,
            level: 1,
            post_id,
            replies: [],
          });
        }
        return async ({ update }) => {
          await update();
          isReplying = false;
        };
      }}
    >
      <Input
        id={`reply-${comment.id}-content-input`}
        label="Content"
        name="content"
        type="text"
        errorText={$page.form?.validationError?.content}
        errorTextId={`reply-${comment.id}-content-error-text`}
      />
      <div>
        <button class="button" disabled={isReplying}>Reply</button>
      </div>
    </form>
    {#if comment.replies.length > 0}
      {#each comment.replies as reply}
        <svelte:self comment={reply} parentId={comment.id} {username} />
      {/each}
    {/if}
  </div>
</div>
