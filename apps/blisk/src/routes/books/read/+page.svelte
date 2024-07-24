<script lang="ts">
  import type { Action } from "svelte/action";
  import { enhance } from "$app/forms";
  import NavLink from "$components/layouts/NavLink.svelte";
  import DatePicker from "$components/DatePicker.svelte";

  const { data, form } = $props();

  let isLoading = $state(false);
  let observer = $state<IntersectionObserver | null>(null);
  let activeId = $state<string | null>(null);

  $effect(() => {
    observer = new IntersectionObserver(
      (entries) => {
        if (entries.length > 0) {
          activeId = `#${entries[0].target.id}`;
        } else {
          activeId = null;
        }
      },
      {
        rootMargin: "0% 0% 0% 0%",
      }
    );

    return () => {
      observer?.disconnect();
      observer = null;
    };
  });

  const trackActive: Action<HTMLElement, IntersectionObserver | null> = (node, initialObserver) => {
    let observer = initialObserver;
    observer?.observe(node);
    return {
      update(newObserver) {
        observer?.unobserve(node);
        (observer = newObserver)?.observe(node);
      },
      destroy() {
        observer?.unobserve(node);
      },
    };
  };

  const links = [
    ["#read-book-starting", "Starting date"],
    ["#read-book-ending", "Ending date"],
  ] as const;
</script>

<div class="h-full w-full p-2 md:py-8">
  <h1 class="h2 mb-8" use:trackActive={observer}>Read a book</h1>
  <div class="relative flex w-full flex-col gap-4 xl:flex-row xl:justify-between">
    <nav class="top-0 flex shrink-0 flex-col gap-[5px] xl:sticky xl:max-h-dvh xl:w-[200px] print:hidden" aria-label="Table of contents">
      {#each links as [href, title]}
        <NavLink {href} isActive={activeId === href} textCenter={false}>{title}</NavLink>
      {/each}
    </nav>
    <form
      method="POST"
      class="flex flex-1 flex-col gap-3"
      use:enhance={() => {
        isLoading = true;
        return async ({ update }) => {
          await update();
          isLoading = false;
        };
      }}
    >
      <DatePicker
        name="startingDate"
        label="Starting date"
        id="read-book-starting"
        required
        value={data.now}
        min={data.now}
        max={data.oneYearLater}
        errorTextId="read-book-starting-error"
        errorText={form?.validationError?.startingDate}
        actions={[[trackActive, observer]]}
      />
      <DatePicker
        name="endingDate"
        label="Ending date"
        id="read-book-ending"
        required
        value={data.now}
        min={data.now}
        max={data.oneYearLater}
        errorTextId="read-book-ending-error"
        errorText={form?.validationError?.endingDate}
        actions={[[trackActive, observer]]}
      />
      <div class="flex w-full flex-row-reverse items-center gap-4">
        <button class="button !px-20 !py-3" type="submit" disabled={isLoading}>Start reading</button>
        <a class="button light !px-20 !py-3" href="/books">Cancel</a>
        {#if !!form?.error}
          {#if typeof form.error === "string"}
            <p class="text-error-light dark:text-error-dark">{form.error}</p>
          {:else}
            <div class="flex flex-row gap-2">
              {#each form.error as error}
                <p class="text-error-light dark:text-error-dark">{error}</p>
              {/each}
            </div>
          {/if}
        {/if}
      </div>
    </form>
  </div>
</div>
