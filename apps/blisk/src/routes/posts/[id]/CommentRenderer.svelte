<script lang="ts">
  import { enhance } from "$app/forms";
  import type { Comment } from "$lib/types";
  import CommentForm from "./CommentForm.svelte";

  interface CommentProps {
    comment: Comment;
    username: string | undefined;
  }

  const { comment, username }: CommentProps = $props();
</script>

<li>
  <div class="mb-3 rounded-lg border border-neutral-400 bg-white px-2.5 py-2.5 shadow-md dark:border-neutral-700">
    <div class="text-comment text-sm">{comment.author_name} - {comment.id}</div>
    <div>{comment.content}</div>
    <div class="flex flex-row gap-2">
      <form method="POST" action="?/react" use:enhance>
        <input name="forType" value="comment" hidden />
        <input name="commentId" value={"" + comment.id} hidden />
        <input name="reactionType" value="like" hidden />
        <button type="submit">Like</button>
      </form>
      <form method="POST" action="?/react" use:enhance>
        <input name="forType" value="comment" hidden />
        <input name="commentId" value={"" + comment.id} hidden />
        <input name="reactionType" value="angry" hidden />
        <button type="submit">Dislike</button>
      </form>
    </div>
  </div>
  <div class="flex flex-col gap-3 pl-3">
    <details>
      <summary>Comment</summary>
      <CommentForm parentId={comment.id} updateComments={(newComment) => comment.replies.unshift(newComment)} />
    </details>
    <ul>
      {#if comment.replies && comment.replies.length > 0}
        {#each comment.replies as reply}
          <svelte:self comment={reply} {username} />
        {/each}
      {/if}
    </ul>
  </div>
</li>
