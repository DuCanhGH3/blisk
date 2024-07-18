import type { Action, ActionReturn } from "svelte/action";

export const combineActions = <T extends HTMLElement = HTMLElement, P extends any = any>(node: T, actions: [Action<T, P>, P][] | undefined) => {
  let actionReturns: (ActionReturn<P> | void)[] = [];

  if (actions) {
    for (const actionEntry of actions) {
      const action = Array.isArray(actionEntry) ? actionEntry[0] : actionEntry;
      actionReturns.push(action(node, actionEntry[1]));
    }
  }

  return {
    update(actions: [Action<T, P>, P][]) {
      if ((actions?.length ?? 0) !== actionReturns.length) {
        throw new Error("You must not change the length of the actions array.");
      }

      if (actions) {
        for (let i = 0; i < actions.length; i++) {
          const returnEntry = actionReturns[i];
          if (returnEntry?.update) {
            const actionEntry = actions[i];
            if (Array.isArray(actionEntry) && actionEntry.length > 1) {
              returnEntry.update(actionEntry[1]);
            } else {
              returnEntry.update(undefined!);
            }
          }
        }
      }
    },

    destroy() {
      for (const actionReturn of actionReturns) {
        actionReturn?.destroy?.();
      }
    },
  };
};
