<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { clsx } from "$lib/clsx";
  // import { hotkeys } from "$lib/hotkeys.svelte";
  import type { ReactionFor, ReactionType } from "$lib/types";
  import { isValidReaction } from "$lib/utils";
  import type { HTMLFormAttributes } from "svelte/elements";
  import Like from "./icons/reactions/Like.svelte";
  import Heart from "./icons/reactions/Heart.svelte";
  import Haha from "./icons/reactions/Haha.svelte";
  import Wow from "./icons/reactions/Wow.svelte";
  import Sad from "./icons/reactions/Sad.svelte";
  import Angry from "./icons/reactions/Angry.svelte";
  import { OPTIMISTIC_ID } from "$lib/constants";

  interface ReactionBarProps extends HTMLFormAttributes {
    forId: number;
    forType: ReactionFor;
    updateReaction(type: ReactionType): void;
    revertReaction(): void;
  }

  const { forId, forType, updateReaction, revertReaction, class: className, ...props }: ReactionBarProps = $props();

  let isProcessing = $state(false);

  const isForOptimisticComment = $derived(forType === "comment" && forId === OPTIMISTIC_ID);

  const disabled = $derived(isForOptimisticComment || isProcessing);

  // hotkeys([
  //   [
  //     "Escape",
  //     () => {
  //       console.log("sus");
  //     },
  //   ],
  // ]);
</script>

<form
  method="POST"
  action="?/react"
  class={clsx(
    "dark:bg-neutral-915 border-border-light dark:border-border-dark z-10 flex flex-row gap-2 rounded-full border bg-white p-1 shadow-md",
    className
  )}
  use:enhance={({ formData }) => {
    isProcessing = true;
    const reactionType = formData.get("reactionType");
    if (reactionType && isValidReaction(reactionType)) {
      updateReaction(reactionType);
    }
    return async ({ result }) => {
      if (result.type === "error" || result.type === "failure") {
        revertReaction();
      } else if (result.type === "redirect") {
        goto(result.location, { invalidateAll: true });
      }
      isProcessing = false;
    };
  }}
  {...props}
>
  <input type="hidden" name="forId" value={forId} />
  <input type="hidden" name="forType" value={forType} />
  <button class="react-button" type="submit" name="reactionType" value="like" aria-label="Like" {disabled}>
    <Like aria-hidden="true" tabindex={-1} />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="love" aria-label="Love" {disabled}>
    <Heart aria-hidden="true" tabindex={-1} />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="laugh" aria-label="Haha" {disabled}>
    <Haha aria-hidden="true" tabindex={-1} />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="wow" aria-label="Wow" {disabled}>
    <Wow animatable={false} aria-hidden="true" tabindex={-1} />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="sad" aria-label="Sad" {disabled}>
    <Sad aria-hidden="true" tabindex={-1} />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="angry" aria-label="Angry" {disabled}>
    <Angry aria-hidden="true" tabindex={-1} />
  </button>
</form>
