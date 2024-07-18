<script lang="ts">
  import type { Action } from "svelte/action";
  import { Set } from "svelte/reactivity";
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";
  import NavLink from "$components/layouts/NavLink.svelte";
  import Textarea from "$components/Textarea.svelte";

  const { form } = $props();

  let isLoading = $state(false);
  let observer = $state<IntersectionObserver | null>(null);
  let activeIds = $state<Set<string>>(new Set());

  $effect(() => {
    observer = new IntersectionObserver(
      (entries) => {
        activeIds.clear();
        for (const entry of entries) {
          activeIds.add(`#${entry.target.id}`);
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
    ["#create-book-title", "Title"],
    ["#create-book-content", "Content"],
  ] as const;
</script>

<div class="h-full w-full p-2 md:py-8">
  <h1 class="mb-8" use:trackActive={observer}>Create a book draft</h1>
  <div class="relative flex w-full flex-col gap-4 xl:flex-row xl:justify-between">
    <nav class="top-0 flex shrink-0 flex-col gap-[5px] xl:sticky xl:max-h-dvh xl:w-[200px] print:hidden" aria-label="Table of contents">
      {#each links as [href, title]}
        <NavLink {href} isActive={activeIds.has(href)} textCenter={false}>{title}</NavLink>
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
      <Input
        name="title"
        label="Title"
        id="create-book-title"
        required
        errorTextId="create-book-title-error"
        errorText={form?.validationError?.title}
        actions={[[trackActive, observer]]}
      />
      <Textarea
        name="summary"
        label="Synopsis"
        id="create-book-content"
        rows={5}
        required
        errorTextId="create-book-content-error"
        errorText={form?.validationError?.summary}
        actions={[[trackActive, observer]]}
      />
      <div class="flex w-full flex-row-reverse gap-4 items-center">
        <button class="button !px-20 !py-3" type="submit" disabled={isLoading}>Create</button>
        <a class="button light !px-20 !py-3" href="/books">Cancel</a>
        {#if form?.error}
          <p class="text-error">{form.error}</p>
        {/if}
      </div>
    </form>
  </div>
</div>
