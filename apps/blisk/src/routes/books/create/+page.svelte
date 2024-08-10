<script lang="ts">
  import type { Action } from "svelte/action";
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";
  import NavLink from "$components/layouts/NavLink.svelte";
  import Textarea from "$components/Textarea.svelte";

  const { form } = $props();

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
    ["#create-book-title", "Book title"],
    ["#create-book-pages", "Number of pages"],
    ["#create-book-content", "Book synopsis"],
  ] as const;
</script>

<div class="h-full w-full p-2 md:py-8">
  <h1 class="h2 mb-8" use:trackActive={observer}>Create a book draft</h1>
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
      <Input
        name="title"
        label="Book title"
        id="create-book-title"
        required
        errorTextId="create-book-title-error"
        errorText={form?.validationError?.title}
        actions={[[trackActive, observer]]}
      />
      <Input
        type="number"
        name="pages"
        id="create-book-pages"
        label="Number of pages"
        errorTextId="create-post-pages-error"
        errorText={form?.validationError?.pages}
        required
      />
      <Textarea
        name="summary"
        label="Book synopsis"
        id="create-book-content"
        rows={5}
        required
        errorTextId="create-book-content-error"
        errorText={form?.validationError?.summary}
        actions={[[trackActive, observer]]}
      />
      <div class="flex w-full flex-row-reverse items-center gap-4">
        <button class="button !px-20 !py-3" type="submit" disabled={isLoading}>Create</button>
        <a class="button light !px-20 !py-3" href="/books">Cancel</a>
        {#if form?.error}
          <p class="text-error-light dark:text-error-dark">{form.error}</p>
        {/if}
      </div>
    </form>
  </div>
</div>
