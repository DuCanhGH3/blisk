<script lang="ts">
  import { enhance } from "$app/forms";
  import type { Comment } from "$lib/types";
  import { OPTIMISTIC_ID } from "$lib/constants";
  import { page } from "$app/stores";
  import Input from "$components/Input.svelte";

  interface CommentFormProps {
    disabled?: boolean;
    parentId: number | null;
    updateComments(newComment: Comment): void;
  }

  const { disabled = false, updateComments, parentId }: CommentFormProps = $props();
  const idPrefix = $derived(parentId !== null ? `reply-${parentId}` : "comment");
  let isCommenting = $state(false);
</script>

<form
  method="POST"
  action={`?/comment${parentId !== null ? `&parentId=${parentId}` : ""}`}
  class="flex flex-col gap-2"
  use:enhance={({ formData }) => {
    isCommenting = true;
    const content = formData.get("content");
    const author_name = $page.data.user?.name;
    const post_id = Number.parseInt($page.params.id);
    let comment = $state<Comment | null>(null);
    if (typeof author_name === "string" && typeof content === "string" && !Number.isNaN(post_id)) {
      comment = {
        id: OPTIMISTIC_ID,
        path: "Top",
        content,
        author_name,
        level: 1,
        post_id,
        replies: [],
      } satisfies Comment;
      updateComments(comment);
    }
    return async ({ result, update }) => {
      if (result.type === "success" && comment !== null && typeof result.data?.id === "number") {
        comment.id = result.data.id;
      } else {
        await update();
      }
      isCommenting = false;
    };
  }}
>
  <Input
    id={`${idPrefix}-content-input`}
    {disabled}
    label="Content"
    name="content"
    type="text"
    errorText={$page.form?.validationError?.content}
    errorTextId={`${idPrefix}-content-error-text`}
  />
  <div>
    <button class="button" disabled={disabled || isCommenting}>{parentId !== null ? "Reply" : "Comment"}</button>
  </div>
</form>
