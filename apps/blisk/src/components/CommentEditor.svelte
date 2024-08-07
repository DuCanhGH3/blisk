<script lang="ts">
  import X from "$components/icons/X.svelte";
  import Comment from "$components/icons/Comment.svelte";
  import type { ClientComment } from "$lib/types";
  import Textarea from "./Textarea.svelte";

  interface CommentEditorProps {
    comment: ClientComment;
    toggleEditingMode(): void;
  }

  const { comment, toggleEditingMode }: CommentEditorProps = $props();
</script>

<form method="POST" action="?/editComment" class="relative p-1">
  <input type="hidden" name="id" value={comment.id} />
  <Textarea
    id="{comment.id}-edit-content-input"
    class="peer"
    label="Content"
    name="content"
    bind:value={comment.editText}
    rows={5}
    errorText={undefined}
    errorTextId="{comment.id}-edit-content-error-text"
  />
  <div class="absolute bottom-2 right-2 flex flex-row gap-2">
    <button type="button" class="button light !rounded-full !p-2" onclick={toggleEditingMode}>
      <X width={20} height={20} class="h-auto w-5" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Cancel</span>
    </button>
    <button type="submit" class="button !rounded-full !p-2">
      <Comment width={20} height={20} class="h-auto w-5" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Edit</span>
    </button>
  </div>
</form>
