import type { SVGAttributes } from "svelte/elements";

export interface ReactionProps extends Omit<SVGAttributes<SVGElement>, "xmlns" | "viewBox" | "fill"> {
  animatable?: boolean;
}

export interface IconProps extends Omit<SVGAttributes<SVGElement>, "xmlns" | "viewBox"> {}
