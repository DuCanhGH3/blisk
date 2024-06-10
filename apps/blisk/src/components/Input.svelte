<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  import { clsx } from "$lib/clsx";

  interface InputProps extends Omit<HTMLInputAttributes, "placeholder"> {
    label: string;
    id: string;
    errorTextId?: string;
    errorText?: string | string[];
  }

  const { label, id, errorTextId, errorText, ...rest }: InputProps = $props();
</script>

<div class="relative">
  <input
    {id}
    class={clsx(
      "input block h-[44px] w-full rounded-lg px-2.5 pt-2.5 text-sm shadow-md transition-opacity disabled:opacity-50",
      "focus:border-accent-light dark:focus:border-accent-dark border border-neutral-400 focus:outline-none dark:border-neutral-700",
      "dark:bg-neutral-1000 bg-white text-black opacity-80 dark:text-white"
    )}
    aria-invalid={!!errorText}
    aria-describedby={errorTextId}
    placeholder=" "
    {...rest}
  />
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
