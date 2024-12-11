<script lang="ts">
  import { page } from "$app/stores";
  import { isLinkActive } from "$lib/isLinkActive";
  import { clsx } from "$lib/clsx";
  import ToggleScheme from "./ToggleScheme.svelte";
  import Menu from "./Menu.svelte";
  import NavLink from "./NavLink.svelte";
  import SearchBar from "./SearchBar.svelte";

  interface SidebarLinkEntry {
    label: string;
    link: string;
  }

  let sidebar = $state<HTMLElement>(null!);

  const links = $derived(
    (
      [
        { label: "Home", link: "/" },
        { label: "Books", link: "/books" },
        { label: "Posts", link: "/posts" },
        ...($page.data.user ? [] : [{ label: "Login", link: "/auth/login" }]),
      ] satisfies SidebarLinkEntry[]
    ).map(({ link, ...rest }) => ({
      link,
      ...rest,
      isActive: isLinkActive(link, $page.url.pathname),
    }))
  );

  $effect(() => {
    $page.url.pathname;
    sidebar?.hidePopover();
  });
</script>

<svelte:window
  onresize={() => {
    sidebar?.hidePopover();
  }}
/>

<aside
  bind:this={sidebar}
  id="nav"
  class={clsx(
    "transition-discrete z-321032 fixed bottom-0 left-0 top-[unset] h-[85dvh] w-dvw rounded-t-2xl px-8 transition-[transform,translate,width]",
    "duration-400 p-2 md:sticky md:bottom-[unset] md:top-0 md:flex md:h-dvh md:w-64 md:shrink-0 md:flex-col md:rounded-none md:px-4 lg:w-80",
    "bg-wood-hor dark:bg-dark-wood-hor dark:bg-wood-915 bg-wood-400 md:[background-image:initial]! md:bg-transparent dark:md:bg-transparent",
    "border-border-light dark:border-border-dark text-wood-900 dark:text-wood-200 border-b md:border-b-0"
  )}
  popover="auto"
>
  <button popovertarget="nav" popovertargetaction="hide" class="relative h-11 w-full cursor-pointer md:hidden">
    <span class="absolute left-1/2 h-1 w-1/5 -translate-x-1/2 rounded-md bg-gray-400">
      <span class="sr-only">Close drawer</span>
    </span>
  </button>
  <nav class="transition-colors-opacity z-50 flex h-fit w-full flex-col items-start gap-[5px] duration-100">
    <div class="flex items-start gap-2 md:block md:py-2">
      <a class="shrink-0 select-none px-3 py-2 font-mono text-base" href="/" aria-label="Go to home">blisk</a>
    </div>
    <div class="order-last w-full min-w-0 grow basis-0 pr-2 md:flex-1 md:pr-0">
      <ul class="ml-auto flex w-full grow flex-col items-start gap-[5px] overflow-y-auto">
        {#each links as { label, link, isActive }}
          <li class="w-full">
            <NavLink href={link} textCenter={false} {isActive}>
              {label}
            </NavLink>
          </li>
        {/each}
      </ul>
    </div>
  </nav>
  <div class="absolute bottom-0 left-0 flex w-full flex-row-reverse gap-2 p-2">
    <Menu />
    <ToggleScheme />
    <SearchBar />
  </div>
</aside>
