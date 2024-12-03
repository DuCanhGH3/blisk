<script lang="ts">
  // This component is stateless to work with the virtual scroller.
  // TODO(ducanhgh): When we unmount the component via `comment.is_editing = false`,
  // our focus returns to the beginning of the page. This is not a desired behaviour.
  import X from "$components/icons/X.svelte";
  import CommentIcon from "$components/icons/Comment.svelte";
  import type { Comment } from "$lib/types";
  import { enhance } from "$app/forms";
  import { OPTIMISTIC_ID } from "$lib/constants";
  import { hotkeys } from "$lib/hotkeys.svelte";
  import Textarea from "../Textarea.svelte";
  import Button from "../Button.svelte";

  interface CommentEditorProps {
    comment: Comment;
  }

  const { comment = $bindable() }: CommentEditorProps = $props();
  let form = $state<HTMLFormElement | null>(null);
  let textarea = $state<HTMLTextAreaElement | null>(null);
  let value = $state(comment.edit_text ?? comment.content);
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
          comment.is_editing = false;
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

  const optimistic = (content: string) => {
    comment.old_content = comment.content;
    comment.content = content;
    comment.edit_text = content;
    comment.is_editing = false;
  };

  const revertOptimistic = () => {
    if (comment.old_content) {
      comment.content = comment.old_content;
      comment.is_editing = true;
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
    comment.is_processing_edit = true;
    const content = formData.get("content");
    if (typeof content === "string") {
      // Optimistically update the comment's content.
      optimistic(content);
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
      comment.is_processing_edit = false;
    };
  }}
>
  <input type="hidden" name="id" value={comment.id} />
  <Textarea
    bind:self={textarea}
    bind:value
    id="{comment.id}-edit-content-input"
    class="peer"
    name="content"
    label="Content"
    rows={5}
    oninput={(el) => (comment.edit_text = el.currentTarget.value)}
    errorTextId="{comment.id}-edit-content-error-text"
    errorText={comment.error?.validationError?.content}
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
  <!--
    .error and .validationError don't appear together, so we can reuse `right-[6.25rem]`.
    This seems cursed though. Is there a better way?
  -->
  {#if comment.error?.error}
    <p
      id="{comment.id}-edit-error-text"
      class="text-error-light dark:text-error-dark absolute bottom-2 right-[6.25rem] flex h-[2.375rem] flex-col justify-center gap-2"
      role="alert"
    >
      {comment.error.error}
    </p>
  {/if}
  <div class="absolute bottom-2 right-2 flex flex-row gap-2">
    <Button
      as="button"
      variant="light"
      class="!rounded-full !p-2"
      type="button"
      onclick={() => (comment.is_editing = false)}
      disabled={comment.is_processing_edit}
    >
      <X width={20} height={20} class="h-auto w-5" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Cancel</span>
    </Button>
    <Button as="button" class="!rounded-full !p-2" type="submit" disabled={isCommentOptimistic || comment.is_processing_edit}>
      <CommentIcon width={20} height={20} class="h-auto w-5" aria-hidden="true" tabindex={-1} />
      <span class="sr-only">Edit</span>
    </Button>
  </div>
</form>
