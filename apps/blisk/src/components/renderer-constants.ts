import type { Component } from "svelte";
import type { ReactionType } from "$lib/types";
import Like from "./icons/reactions/Like.svelte";
import Heart from "./icons/reactions/Heart.svelte";
import Haha from "./icons/reactions/Haha.svelte";
import Wow from "./icons/reactions/Wow.svelte";
import Sad from "./icons/reactions/Sad.svelte";
import Angry from "./icons/reactions/Angry.svelte";
import type { IconProps, ReactionProps } from "./icons/types";

export const reactionRender = {
  like: {
    icon: Like,
    label: "Like",
    colors: "bg-blue-100 hover:bg-blue-200 dark:bg-sky-950 dark:hover:bg-sky-900",
  },
  love: {
    icon: Heart,
    label: "Love",
    colors: "bg-rose-100 hover:bg-rose-200 dark:bg-rose-950 dark:hover:bg-rose-900",
  },
  laugh: {
    icon: Haha,
    label: "Haha",
    colors: "bg-yellow-75 hover:bg-yellow-200 dark:bg-yellow-1000 dark:hover:bg-yellow-925",
  },
  wow: {
    icon: Wow,
    label: "Wow",
    colors: "bg-yellow-75 hover:bg-yellow-200 dark:bg-yellow-1000 dark:hover:bg-yellow-925",
  },
  sad: {
    icon: Sad,
    label: "Sad",
    colors: "bg-yellow-75 hover:bg-yellow-200 dark:bg-yellow-1000 dark:hover:bg-yellow-925",
  },
  angry: {
    icon: Angry,
    label: "Angry",
    colors: "bg-orange-200 hover:bg-orange-300 dark:bg-orange-950 dark:hover:bg-orange-900",
  },
} satisfies Record<ReactionType, { icon: Component<ReactionProps>; label: string; colors: string }>;

export const rendererButtonAttributes = {
  width: 24,
  height: 24,
  class: "h-6 w-auto",
  "aria-hidden": "true",
  tabindex: -1,
} satisfies IconProps;