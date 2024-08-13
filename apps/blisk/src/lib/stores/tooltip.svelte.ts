export interface TooltipState {
  id: string | undefined;
  text: string | undefined;
  x: number | undefined;
  y: number | undefined;
  right: boolean;
  bottom: boolean;
  maxHeight: number | undefined;
  closeTooltip(): void;
  timeout: NodeJS.Timeout | undefined;
}

const tooltipDefault = {
  id: undefined,
  text: undefined,
  x: undefined,
  y: undefined,
  right: false,
  bottom: false,
  maxHeight: undefined,
  closeTooltip: undefined!,
  timeout: undefined,
} satisfies TooltipState;

export const tooltip = (() => {
  let tooltipState = $state<TooltipState>(tooltipDefault);

  return {
    get state() {
      return tooltipState;
    },
    set state(value) {
      tooltipState = value;
    },
    reset() {
      tooltipState = tooltipDefault;
    }
  };
})();
