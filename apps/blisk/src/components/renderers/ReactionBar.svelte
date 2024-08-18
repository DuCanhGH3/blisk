<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { clsx } from "$lib/clsx";
  import type { ReactionFor, ReactionType } from "$lib/types";
  import { isValidReaction } from "$lib/utils";
  import type { HTMLFormAttributes } from "svelte/elements";
  import { OPTIMISTIC_ID, VALID_REACTIONS } from "$lib/constants";
  import { reactionRender } from "./renderer-constants";
  import { page } from "$app/stores";

  interface ReactionBarProps extends HTMLFormAttributes {
    currentReaction: ReactionType | null;
    forId: number;
    forType: ReactionFor;
    updateReaction(type: ReactionType | null): void;
    revertReaction(): void;
  }

  const { currentReaction, forId, forType, updateReaction, revertReaction, class: className, ...props }: ReactionBarProps = $props();

  let isProcessing = $state(false);

  const isForOptimisticComment = $derived(forType === "comment" && forId === OPTIMISTIC_ID);

  const disabled = $derived(!$page.data.user || isForOptimisticComment || isProcessing);
</script>

<form
  method="POST"
  action="?/react"
  class={clsx("dark:bg-neutral-915 border-border-light dark:border-border-dark z-10 rounded-full border bg-white p-1 shadow-md", className)}
  use:enhance={({ formData }) => {
    isProcessing = true;
    const reactionType = formData.get("reaction_type");
    if (reactionType) {
      if (isValidReaction(reactionType)) {
        updateReaction(reactionType);
      } else if (reactionType === "cancel") {
        updateReaction(null);
      }
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
  <input type="hidden" name="post_id" value={forId} />
  <input type="hidden" name="for_type" value={forType} />
  <menu class="flex flex-row gap-2">
    {#each VALID_REACTIONS as reactionType}
      {@const mappedRender = reactionRender[reactionType]}
      {@const isCurrent = reactionType === currentReaction}
      {@const Icon = mappedRender.icon}
      <li class="react-button" role="presentation">
        <button
          class="flex h-full w-full items-center justify-center"
          type="submit"
          name="reaction_type"
          value={isCurrent ? "cancel" : reactionType}
          {disabled}
          role="menuitemcheckbox"
          aria-checked={isCurrent}
          aria-label={isCurrent ? `Undo ${mappedRender.label} reaction` : mappedRender.label}
        >
          <Icon animatable={!isCurrent && reactionType !== "wow"} aria-hidden="true" tabindex={-1}>
            {#if isCurrent}
              <path
                d="M8 8m7.25 0a7.25 7.25 0 1 0 -14.5 0a7.25 7.25 0 1 0 14.5 0"
                stroke="currentColor"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path d="M12.242 3.757l-8.485 8.485" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            {/if}
          </Icon>
        </button>
      </li>
    {/each}
  </menu>
</form>
