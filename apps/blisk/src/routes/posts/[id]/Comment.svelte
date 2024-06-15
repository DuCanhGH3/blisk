<script lang="ts">
  import { OPTIMISTIC_ID } from "$lib/constants";
  import type { Comment } from "$lib/types";
  import CommentForm from "./CommentForm.svelte";

  interface CommentProps {
    comment: Comment;
    username: string | undefined;
  }

  const { comment, username }: CommentProps = $props();
</script>

<div>
  <div>{comment.author_name}</div>
  <div>{comment.content}</div>
  <div class="pl-3">
    <CommentForm
      disabled={comment.id === OPTIMISTIC_ID}
      parentId={comment.id}
      updateComments={(newComment) => {
        comment.replies.unshift(newComment);
      }}
    />
    {#if comment.replies.length > 0}
      {#each comment.replies as reply}
        <svelte:self comment={reply} {username} />
      {/each}
    {/if}
  </div>
</div>
