import type { ButtonType } from "$lib/types";

export type DialogState = { title: string; description: string; closeVariant?: ButtonType; closeText: string; onClose(): void } & (
  | { type: "action"; cancelText: string; onCancel(): void }
  | { type: "content" }
);

export const dialog = (() => {
  let dialogState = $state<DialogState | null>(null);

  return {
    get state() {
      return dialogState;
    },
    set state(value: DialogState | null) {
      dialogState = value;
    },
  };
})();
