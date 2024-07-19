<script lang="ts" generics="T extends HTMLElement = HTMLElement, P extends any = any">
  import type { Action } from "svelte/action";
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { combineActions } from "$lib/combineActions";
  import { clsx } from "$lib/clsx";

  interface TextareaProps extends Omit<HTMLTextareaAttributes, "placeholder"> {
    actions?: [Action<T, P>, P][];
    label: string;
    id: string;
    errorTextId?: string;
    errorText?: string | string[];
  }

  const { actions, label, id, errorTextId, errorText, class: className, oninput, ...rest }: TextareaProps = $props();
</script>

<div class="relative w-full">
  <textarea
    {id}
    class={clsx(
      "textarea block min-h-[44px] w-full overflow-hidden rounded-lg px-2.5 pb-2.5 pt-4 text-sm shadow-md transition-opacity disabled:opacity-50",
      "focus:border-accent-light dark:focus:border-accent-dark border-border-light dark:border-border-dark border focus:outline-none",
      "dark:bg-neutral-915 bg-white text-black opacity-80 dark:text-white",
      className
    )}
    aria-invalid={!!errorText}
    aria-describedby={errorTextId}
    placeholder=" "
    oninput={(ev) => (ev.currentTarget.style.height = `${Math.max(44, ev.currentTarget.offsetHeight, ev.currentTarget.scrollHeight)}px`)}
    use:combineActions={actions}
    {...rest}
  ></textarea>
  <label class="label absolute left-2.5 block select-none font-medium transition-all duration-100 ease-in" for={id}>
    {label}
  </label>
</div>
{#if !!errorText && errorTextId}
  {#if typeof errorText === "string"}
    <p class="text-error" id={errorTextId}>{errorText}</p>
  {:else}
    <div id={errorTextId} class="flex flex-col gap-2">
      {#each errorText as error}
        <p class="text-error">{error}</p>
      {/each}
    </div>
  {/if}
{/if}
