import type { ButtonType } from "$lib/types";

export type DialogState = { title: string; description: string; closeVariant?: ButtonType; closeText: string; onClose(): void } & (
  | { type: "action"; cancelText: string; onCancel(): void }
  | { type: "content" }
);

export const dialog = (() => {
  let dialogElement = $state<HTMLDialogElement | null>(null);
  let dialogState = $state<DialogState | null>(null);
  let scroll = 0;

  // TODO(ducanhgh): Currently, our focus returns to top of the
  // page when the focused element is loaded by the virtual scroller
  // and is not in view when the dialog is shown. This is not desired.
  return {
    get state() {
      return dialogState;
    },
    set state(value) {
      dialogState = value;
      if (dialogState !== null) {
        scroll = document.documentElement.scrollTop;
        document.documentElement.style.overflowY = "hidden";
        dialogElement?.showModal();
      } else {
        dialogElement?.close();
        document.documentElement.style.overflowY = "";
        document.documentElement.scrollTo({ top: scroll, behavior: "instant" });
      }
    },
    get element() {
      return dialogElement;
    },
    set element(value) {
      dialogElement = value;
    },
  };
})();
