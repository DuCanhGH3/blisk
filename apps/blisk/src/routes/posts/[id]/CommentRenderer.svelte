<script lang="ts">
  import ThumbUp from "$components/icons/ThumbUp.svelte";
  import type { Comment } from "$lib/types";
  import CommentForm from "./CommentForm.svelte";
  import ReactionBar from "./ReactionBar.svelte";

  interface CommentProps {
    comment: Comment;
    username: string | undefined;
  }

  const { comment, username }: CommentProps = $props();
</script>

<li>
  <div class="box sm">
    <a href={`/users/${comment.author_name}`} class="link text-sm">{comment.author_name}</a>
    <div>{comment.id} - {comment.content}</div>
    <div class="flex flex-row gap-2">
      <span class="flex flex-row gap-1">
        <ThumbUp width={24} height={24} />
        Like
      </span>
      <ReactionBar />
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
