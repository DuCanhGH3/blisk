import type { EasingFunction } from "svelte/transition";

interface UsermenuParams {
  delay?: number;
  duration?: number;
  easing?: EasingFunction;
}

interface UsermenuConfig {
  delay?: number;
  duration?: number;
  easing?: EasingFunction;
  css?: (t: number, u: number) => string;
}

export const usermenu = (node: HTMLElement, { delay = 0, duration = 100, easing }: UsermenuParams): UsermenuConfig => {
  const o = +getComputedStyle(node).opacity;
  return {
    delay,
    duration,
    easing,
    css: (t) => `
            opacity: ${t * o}; 
            transform: scale(calc(0.95 + ${t / 20}));
        `,
  };
};
