<script lang="ts" generics="T extends HTMLElement = HTMLElement, P extends any = any">
  import type { Action } from "svelte/action";
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { combineActions } from "$lib/combineActions";
  import { clsx } from "$lib/clsx";
  import type { RequireFields } from "$lib/types";
  import type { Snippet } from "svelte";
  import TextareaFn from "./TextareaFn.svelte";
  import Heading from "./icons/textarea/Heading.svelte";
  import Bold from "./icons/textarea/Bold.svelte";
  import Italics from "./icons/textarea/Italics.svelte";
  import Quote from "./icons/textarea/Quote.svelte";
  import Code from "./icons/textarea/Code.svelte";
  import Link from "./icons/textarea/Link.svelte";

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
</script>

<div class="box w-full rounded-[15px] p-2 pt-0 shadow-md">
  <div class="flex w-full flex-row flex-wrap items-center justify-between p-1">
    <span>blisk</span>
    <div class="flex flex-row flex-wrap justify-end gap-2">
      <TextareaFn label="Insert heading">
        <Heading width={24} height={24} aria-hidden="true" tabindex={-1} />
      </TextareaFn>
      <TextareaFn label="Insert bold">
        <Bold width={24} height={24} aria-hidden="true" tabindex={-1} />
      </TextareaFn>
      <TextareaFn label="Insert italics">
        <Italics width={24} height={24} aria-hidden="true" tabindex={-1} />
      </TextareaFn>
      <TextareaFn label="Insert quote">
        <Quote width={24} height={24} aria-hidden="true" tabindex={-1} />
      </TextareaFn>
      <TextareaFn label="Insert code">
        <Code width={24} height={24} aria-hidden="true" tabindex={-1} />
      </TextareaFn>
      <TextareaFn label="Insert link">
        <Link width={24} height={24} aria-hidden="true" tabindex={-1} />
      </TextareaFn>
    </div>
  </div>
  <div class="relative w-full">
    <textarea
      {id}
      bind:this={thisRef}
      bind:value
      class={clsx(
        "textarea shadow-xs block min-h-[128px] w-full overflow-hidden rounded-lg px-2.5 pb-2.5 pt-4 text-sm transition-opacity disabled:opacity-50",
        "focus:border-accent-light dark:focus:border-accent-dark border-border-light dark:border-border-dark focus:outline-hidden border",
        "bg-wood-300 dark:bg-wood-800 field-sizing-content opacity-80",
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
