<script lang="ts">
  import { enhance } from "$app/forms";
  import type { Comment } from "$lib/types";
  import { OPTIMISTIC_ID } from "$lib/constants";
  import { page } from "$app/stores";
  import CommentIcon from "./icons/Comment.svelte";
  import Textarea from "./Textarea.svelte";

  interface CommentFormProps {
    parentId: number | null;
    updateReplies(newReply: Comment): void;
  }

  const { parentId, updateReplies }: CommentFormProps = $props();
  const isParentComment = $derived(parentId === null);
  const isParentOptimistic = $derived(parentId === OPTIMISTIC_ID);
  const idPrefix = $derived(parentId !== null ? `reply-${parentId}` : "comment");
  let isProcessing = $state(false);
</script>

<form
  method="POST"
  action="?/comment{!isParentComment ? `&parentId=${parentId}` : ''}"
  class="flex flex-row gap-2"
  use:enhance={({ formData }) => {
    isProcessing = true;
    const content = formData.get("content");
    const author_name = $page.data.user?.name;
    const post_id = Number.parseInt($page.params.id);
    let comment = $state<Comment | null>(null);
    if (typeof author_name === "string" && typeof content === "string" && !Number.isNaN(post_id)) {
      comment = {
        id: OPTIMISTIC_ID,
        content,
        author_name,
        user_reaction: null,
        children: [],
        is_editing: false,
        edit_text: content,
      } satisfies Comment;
      updateReplies(comment);
    }
    return async ({ result, update }) => {
      if (result.type === "success" && comment !== null && typeof result.data?.id === "number") {
        comment.id = result.data.id;
        await update({ reset: true, invalidateAll: false });
      } else {
        await update();
      }
      isProcessing = false;
    };
  }}
>
  <Textarea
    id="{idPrefix}-content-input"
    class="peer"
    disabled={isParentOptimistic}
    label="Content"
    name="content"
    rows={5}
    errorText={$page.form?.validationError?.content}
    errorTextId="{idPrefix}-content-error-text"
  />
  <button class="button absolute bottom-1 right-1 !rounded-full !p-2" disabled={isParentOptimistic || isProcessing}>
    <CommentIcon width={20} height={20} class="h-auto w-5" aria-hidden="true" tabindex={-1} />
    <span class="sr-only">{isParentComment ? "Comment" : "Reply"}</span>
  </button>
</form>
