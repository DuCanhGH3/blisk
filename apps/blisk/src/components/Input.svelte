<script lang="ts" generics="T extends HTMLElement = HTMLElement, P extends any = any">
  import type { Action } from "svelte/action";
  import type { HTMLInputAttributes } from "svelte/elements";
  import { clsx } from "$lib/clsx";
  import { combineActions } from "$lib/combineActions";
  import type { RequireFields } from "$lib/types";
  import type { Snippet } from "svelte";

  interface InputProps extends RequireFields<Omit<HTMLInputAttributes, "placeholder">, "name"> {
    self?: HTMLInputElement | null;
    actions?: [Action<T, P>, P][];
    label: Snippet<[]> | string;
    id: string;
    errorTextId?: string;
    errorText?: string | string[];
  }

  let { self = $bindable(), actions, label, id, errorTextId, errorText, ...rest }: InputProps = $props();
</script>

<div class="relative w-full">
  <input
    bind:this={self}
    {id}
    class={clsx(
      "input block h-[44px] w-full rounded-lg px-2.5 pt-2.5 text-sm shadow-md transition-opacity disabled:opacity-50",
      "focus:border-accent-light dark:focus:border-accent-dark border-border-light dark:border-border-dark border focus:outline-none",
      "bg-wood-300 dark:bg-wood-900 opacity-80"
    )}
    aria-invalid={!!errorText}
    aria-describedby={errorText ? errorTextId : undefined}
    placeholder=" "
    use:combineActions={actions}
    {...rest}
  />
  <label class="label absolute left-2.5 flex select-none flex-row items-center gap-2 font-medium transition-all duration-100 ease-in" for={id}>
    {#if typeof label === "string"}
      {label}
    {:else}
      {@render label()}
    {/if}
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
