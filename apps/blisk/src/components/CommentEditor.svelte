<script lang="ts">
  // This component is stateless to work with the virtual scroller.
  import X from "$components/icons/X.svelte";
  import Comment from "$components/icons/Comment.svelte";
  import type { ClientComment } from "$lib/types";
  import { enhance } from "$app/forms";
  import { OPTIMISTIC_ID } from "$lib/constants";
  import Textarea from "./Textarea.svelte";
  import { hotkeys } from "$lib/hotkeys.svelte";

  interface CommentEditorProps {
    comment: ClientComment;
  }

  const { comment = $bindable() }: CommentEditorProps = $props();
  let form = $state<HTMLFormElement | null>(null);
  let textarea = $state<HTMLTextAreaElement | null>(null);
  const isCommentOptimistic = $derived(comment.id === OPTIMISTIC_ID);

  // Escape: remove focus from textarea
  // Enter: request submit if keyboard not focused on textarea
  hotkeys([
    [
      "Escape",
      () => {
        if (textarea && document.activeElement === textarea) {
          textarea.blur();
        } else {
          comment.isEditing = false;
        }
      },
      { tagsToIgnore: [] },
    ],
    [
      "Enter",
      () => {
        form?.requestSubmit();
      },
      { tagsToIgnore: ["INPUT", "TEXTAREA", "SELECT"] },
    ],
  ]);

  const optimisticUpdate = (content: string) => {
    comment.oldContent = comment.content;
    comment.content = content;
    comment.editText = content;
    comment.isEditing = false;
  };

  const revertOptimistic = () => {
    if (comment.oldContent) {
      comment.content = comment.oldContent;
      comment.isEditing = true;
    }
  };
</script>

<form
  bind:this={form}
  method="POST"
  action="?/editComment"
  class="relative p-1"
  use:enhance={({ formData }) => {
    // This function's return callback will still be called
    // even when the component is unmounted.
    comment.isProcessingEdit = true;
    const content = formData.get("content");
    if (typeof content === "string") {
      // Optimistically update the comment's content.
      optimisticUpdate(content);
    }
    return async ({ result, update }) => {
      if (result.type === "success") {
        await update({ reset: true, invalidateAll: false });
      } else if (result.type === "error") {
        revertOptimistic();
        await update();
      } else if (result.type === "failure") {
        revertOptimistic();
        comment.error = result.data as typeof comment.error;
      } else {
        await update();
      }
      comment.isProcessingEdit = false;
    };
  }}
>
  <input type="hidden" name="id" value={comment.id} />
  <Textarea
    bind:self={textarea}
    bind:value={comment.editText}
    id="{comment.id}-edit-content-input"
    class="peer"
    label="Content"
    name="content"
    rows={5}
    errorText={comment.error?.validationError?.content}
    errorTextId="{comment.id}-edit-content-error-text"
  >
    {#snippet errorRenderer({ errorTextId, errorText })}
      <div id={errorTextId} class="absolute bottom-2 right-[6.25rem] flex h-[2.375rem] flex-col justify-center gap-2">
        {#if typeof errorText === "string"}
          <p class="text-error-light dark:text-error-dark">{errorText}</p>
        {:else}
          {#each errorText as error}
            <p class="text-error-light dark:text-error-dark">{error}</p>
          {/each}
        {/if}
      </div>
    {/snippet}
  </Textarea>
  <div class="absolute bottom-2 right-2 flex flex-row gap-2">
    <button type="button" class="button light !rounded-full !p-2" onclick={() => (comment.isEditing = false)} disabled={comment.isProcessingEdit}>
      <X width={20} height={20} class="h-auto w-5" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Cancel</span>
    </button>
    <button type="submit" class="button !rounded-full !p-2" disabled={isCommentOptimistic || comment.isProcessingEdit}>
      <Comment width={20} height={20} class="h-auto w-5" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Edit</span>
    </button>
  </div>
</form>
