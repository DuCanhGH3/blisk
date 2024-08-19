<script lang="ts">
  import { clsx } from "$lib/clsx";
  import type { HTMLFieldsetAttributes } from "svelte/elements";

  interface SelectProps extends HTMLFieldsetAttributes {
    id: string;
    label: string;
    options: { id: any; name: string }[];
  }

  const { id, label, options, ...props }: SelectProps = $props();
</script>

<fieldset {id} {...props}>
  <legend class="mb-2 text-sm font-medium">Categories</legend>
  <div class="flex flex-row gap-2">
    {#each options as option}
      <div role="presentation">
        <input type="checkbox" class="peer sr-only" id="{id}-{option.id}" name="categories" value={option.id} />
        <label
          class={clsx(
            "select-none rounded-md px-2 py-1 transition-colors duration-100",
            "bg-neutral-250 cursor-pointer dark:bg-neutral-800",
            "peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-white peer-focus:ring-offset-2 peer-focus:ring-offset-gray-800",
            "peer-checked:bg-accent-light dark:peer-checked:bg-accent-dark peer-checked:text-white dark:peer-checked:text-wood-900"
          )}
          for="{id}-{option.id}"
        >
          {option.name}
        </label>
      </div>
    {/each}
  </div>
</fieldset>
