<script lang="ts">
  import type { Action } from "svelte/action";
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";
  import NavLink from "$components/layouts/NavLink.svelte";
  import Textarea from "$components/Textarea.svelte";
  import Button from "$components/Button.svelte";
  import Select from "$components/Select.svelte";
  import FilePicker from "$components/FilePicker.svelte";

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
    ["#create-book-title", "Book title"],
    ["#create-book-slug", "Slug"],
    ["#create-book-pages", "Number of pages"],
    ["#create-book-content", "Book synopsis"],
    ["#create-book-category", "Categories"],
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
      enctype="multipart/form-data"
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
        name="slug"
        label="Slug (where to access the book)"
        id="create-book-slug"
        required
        errorTextId="create-book-slug-error"
        errorText={form?.validationError?.slug}
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
      <FilePicker
        id="create-book-cover"
        name="cover_image"
        errorText={form?.validationError?.cover_image}
        errorTextId="create-book-cover-error"
        fileType="image"
      />
      <FilePicker
        id="create-book-spine"
        name="spine_image"
        errorText={form?.validationError?.spine_image}
        errorTextId="create-book-spine-error"
        fileType="image"
      />
      <Select id="create-book-category" legends={[{ name: "categories", label: "Categories", options: data.categories }]} />
      <div class="flex w-full flex-row-reverse items-center gap-4">
        <Button as="button" class="!px-20 !py-3" type="submit" disabled={isLoading}>Create</Button>
        <Button as="a" class="!px-20 !py-3" variant="light" href="/books">Cancel</Button>
        {#if form?.error}
          <p class="text-error-light dark:text-error-dark" role="alert">{form.error}</p>
        {/if}
      </div>
    </form>
  </div>
</div>
