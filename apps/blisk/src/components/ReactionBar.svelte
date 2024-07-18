<script lang="ts">
  import { enhance } from "$app/forms";
  import { clsx } from "$lib/clsx";
  // import { hotkeys } from "$lib/hotkeys.svelte";
  import type { ReactionFor } from "$lib/types";
  import type { HTMLFormAttributes } from "svelte/elements";
  import Like from "./icons/reactions/Like.svelte";
  import Heart from "./icons/reactions/Heart.svelte";
  import Haha from "./icons/reactions/Haha.svelte";
  import Wow from "./icons/reactions/Wow.svelte";
  import Sad from "./icons/reactions/Sad.svelte";
  import Angry from "./icons/reactions/Angry.svelte";

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
    "dark:bg-neutral-915 border-border-light dark:border-border-dark z-10 flex flex-row gap-2 rounded-full border bg-white p-1 shadow-md",
    className
  )}
  use:enhance
  {...props}
>
  <input type="hidden" name="forId" value={forId} />
  <input type="hidden" name="forType" value={forType} />
  <button class="react-button" type="submit" name="reactionType" value="like">
    <Like />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="love">
    <Heart />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="laugh">
    <Haha />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="wow">
    <Wow animatable={false} />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="sad">
    <Sad />
  </button>
  <button class="react-button" type="submit" name="reactionType" value="angry">
    <Angry />
  </button>
</form>
