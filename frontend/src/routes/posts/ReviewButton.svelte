<script lang="ts">
  import { page } from "$app/stores";
  import { clsx } from "$lib/clsx";
  import type { Book } from "$lib/types";
  import { getProfilePicture } from "$lib/utils";

  interface ReviewButtonProps {
    book?: Pick<Book, "title" | "name">;
  }

  const { book }: ReviewButtonProps = $props();
</script>

{#if $page.data.user}
  <div class="box flex flex-row gap-2 rounded-[29px] p-2.5 shadow-md">
    <a href="/users/{$page.data.user.name}">
      <img
        src={getProfilePicture($page.data.user.picture ?? null)}
        class="border-border-light dark:border-border-dark size-10 select-none rounded-full border shadow-lg"
        width={40}
        height={40}
        loading="lazy"
        decoding="async"
        alt="Your profile"
      />
    </a>
    <a
      href="/posts/create{book ? `?book=${book.name}` : ''}"
      class={clsx(
        "text-comment flex h-10 w-full items-center rounded-[20px] border px-2.5 transition-colors duration-100",
        "border-border-light dark:border-border-dark dark:bg-wood-800 bg-wood-300 hover:bg-wood-200 dark:hover:bg-wood-750"
      )}
    >
      {book ? `Done reading ${book.title}? Review it!` : "Reviewing a book?"}
    </a>
  </div>
{/if}
