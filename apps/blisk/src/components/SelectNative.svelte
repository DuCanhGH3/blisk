<script lang="ts" generics="T extends string | null">
  import type { HTMLSelectAttributes } from "svelte/elements";

  import { clsx } from "$lib/clsx";

  interface SelectNativeProps extends Omit<HTMLSelectAttributes, "class"> {
    id: string;
    label: string;
    values: readonly [T, string][];
    initialValue?: T | null;
  }

  const { id, label, values, initialValue, ...rest }: SelectNativeProps = $props();
</script>

<div class="relative">
  <label
    for={id}
    class="absolute left-2.5 top-0.5 z-[2] block select-none text-xs font-medium text-neutral-700 transition-all duration-100 ease-in dark:text-gray-300"
  >
    {label}
  </label>
  <select
    {id}
    class={clsx(
      "transition-colors-opacity z-[1] block h-[44px] w-full rounded-lg px-2.5 pt-2.5 text-sm shadow-md duration-100 disabled:opacity-50",
      "focus:border-accent-light dark:focus:border-accent-dark border-border-light dark:border-border-dark border focus:outline-none",
      "bg-wood-300 dark:bg-wood-800 opacity-80"
    )}
    value={initialValue}
    {...rest}
  >
    {#each values as [value, name]}
      <option {value}>{name}</option>
    {/each}
  </select>
</div>
