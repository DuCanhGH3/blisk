<script lang="ts" generics="T extends HTMLElement = HTMLElement, P extends any = any">
  import type { Action } from "svelte/action";
  import type { HTMLInputAttributes } from "svelte/elements";
  import { clsx } from "$lib/clsx";
  import { combineActions } from "$lib/combineActions";
  import type { RequireFields } from "$lib/types";

  interface DatePickerProps extends RequireFields<Omit<HTMLInputAttributes, "type" | "placeholder">, "name"> {
    actions?: [Action<T, P>, P][];
    label: string;
    id: string;
    errorTextId?: string;
    errorText?: string | string[];
  }

  const { actions, label, id, errorTextId, errorText, ...rest }: DatePickerProps = $props();
</script>

<div class="relative w-full">
  <div
    class={clsx(
      "datepicker block h-[44px] w-full rounded-lg px-2.5 pb-2.5 pt-4 text-sm shadow-md transition-opacity disabled:opacity-50",
      "focus:border-accent-light dark:focus:border-accent-dark border-border-light dark:border-border-dark border focus:outline-none",
      "bg-wood-300 dark:bg-wood-800 opacity-80"
    )}
  >
    <input
      {id}
      type="date"
      class="bg-wood-300 dark:bg-wood-800 opacity-80"
      aria-invalid={!!errorText}
      aria-describedby={errorText ? errorTextId : undefined}
      placeholder=" "
      use:combineActions={actions}
      {...rest}
    />
  </div>
  <label class="label absolute left-2.5 block select-none font-medium transition-all duration-100 ease-in" for={id}>
    {label}
  </label>
</div>
{#if !!errorText && errorTextId}
  {#if typeof errorText === "string"}
    <p class="text-error-light dark:text-error-dark" id={errorTextId}>{errorText}</p>
  {:else}
    <div id={errorTextId} class="flex flex-col gap-2">
      {#each errorText as error}
        <p class="text-error-light dark:text-error-dark">{error}</p>
      {/each}
    </div>
  {/if}
{/if}
