<script lang="ts">
  import { enhance } from "$app/forms";
  import Svg from "$components/Svg.svelte";
  import { clsx } from "$lib/clsx";
  // import { hotkeys } from "$lib/hotkeys.svelte";
  import type { ReactionFor } from "$lib/types";
  import type { HTMLFormAttributes } from "svelte/elements";
  import Like from "./icons/reactions/Like.svelte";
  import Heart from "./icons/reactions/Heart.svelte";
  import Haha from "./icons/reactions/Haha.svelte";
  import Wow from "./icons/reactions/Wow.svelte";

  interface ReactionBarProps extends HTMLFormAttributes {
    forId: number;
    forType: ReactionFor;
  }

  const { forId, forType, class: className, ...props }: ReactionBarProps = $props();

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
    "dark:bg-neutral-915 border-border-light dark:border-border-dark flex flex-row gap-2 rounded-full border bg-white p-1 shadow-md",
    className
  )}
  use:enhance
  {...props}
>
  <input type="hidden" name="forId" value={forId} />
  <input type="hidden" name="forType" value={forType} />
  <button type="submit" name="reactionType" value="like">
    <Like width="36" height="36" />
  </button>
  <button type="submit" name="reactionType" value="love">
    <Heart width="36" height="36" />
  </button>
  <button type="submit" name="reactionType" value="laugh">
    <Haha width="36" height="36" />
  </button>
  <button type="submit" name="reactionType" value="wow">
    <Wow width="36" height="36" />
  </button>
</form>
