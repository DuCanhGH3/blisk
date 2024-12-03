import type { SVGAttributes } from "svelte/elements";

export interface IconProps extends Omit<SVGAttributes<SVGElement>, "xmlns" | "viewBox"> {}

export interface ReactionProps extends Omit<IconProps, "fill"> {
  animatable?: boolean;
}
