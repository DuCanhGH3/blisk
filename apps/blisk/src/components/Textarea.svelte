<script lang="ts" generics="T extends HTMLElement = HTMLElement, P extends any = any">
  import type { Action } from "svelte/action";
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { combineActions } from "$lib/combineActions";
  import { clsx } from "$lib/clsx";
  import type { RequireFields } from "$lib/types";
  import type { Snippet } from "svelte";

  interface TextareaProps extends RequireFields<Omit<HTMLTextareaAttributes, "placeholder">, "name"> {
    self?: HTMLTextAreaElement | null;
    actions?: [Action<T, P>, P][];
    label: string;
    id: string;
    errorTextId?: string;
    errorText?: string | string[];
    errorRenderer?: Snippet<[{ errorTextId: string; errorText: string | string[] }]>;
  }

  let {
    self: thisRef = $bindable(),
    value = $bindable(),
    id,
    class: className,
    actions,
    label,
    errorTextId,
    errorText,
    errorRenderer,
    ...rest
  }: TextareaProps = $props();

  const updateHeight = (element: HTMLTextAreaElement) => {
    element.style.height = `${Math.max(128, element.offsetHeight, element.scrollHeight)}px`;
  };

  $effect(() => {
    value;
    if (thisRef) {
      updateHeight(thisRef);
    }
  });
</script>

<div class="relative w-full">
  <textarea
    {id}
    bind:this={thisRef}
    bind:value
    class={clsx(
      "textarea block min-h-[128px] w-full overflow-hidden rounded-lg px-2.5 pb-2.5 pt-4 text-sm shadow-md transition-opacity disabled:opacity-50",
      "focus:border-accent-light dark:focus:border-accent-dark border-border-light dark:border-border-dark border focus:outline-none",
      "dark:bg-neutral-915 bg-white text-wood-950 opacity-80 dark:text-white",
      className
    )}
    aria-invalid={!!errorText}
    aria-describedby={errorText ? errorTextId : undefined}
    placeholder=" "
    use:combineActions={actions}
    {...rest}
  ></textarea>
  <label class="label absolute left-2.5 block select-none font-medium transition-all duration-100 ease-in" for={id}>
    {label}
  </label>
</div>
{#if !!errorText && errorTextId}
  {#if errorRenderer}
    {@render errorRenderer({ errorTextId, errorText })}
  {:else if typeof errorText === "string"}
    <p class="text-error-light dark:text-error-dark" id={errorTextId}>{errorText}</p>
  {:else}
    <div id={errorTextId} class="flex flex-col gap-2">
      {#each errorText as error}
        <p class="text-error-light dark:text-error-dark">{error}</p>
      {/each}
    </div>
  {/if}
{/if}
