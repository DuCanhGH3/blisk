<script lang="ts">
  import { page } from "$app/stores";
  import { clsx } from "$lib/clsx";
  import { isLinkActive } from "$lib/isLinkActive";
  import { BREAKPOINTS } from "$lib/constants";

  import NavLink from "./NavLink.svelte";
  import NavToggleScheme from "./NavToggleScheme.svelte";
  import { LOGGED_IN_LINKS, GENERAL_LINKS } from "./navbar-constants";
  import NavUsermenu from "./NavUsermenu.svelte";
  import SearchBar from "./SearchBar.svelte";

  let { height = $bindable(0) }: { height?: number } = $props();

  let menuCheckbox = $state<HTMLInputElement | null>(null);

  $effect(() => {
    $page.url.pathname;
    const isMenuOpen = window.innerWidth >= BREAKPOINTS.md;
    if (menuCheckbox) {
      menuCheckbox.checked = isMenuOpen;
      menuCheckbox.ariaExpanded = isMenuOpen ? "true" : "false";
    }
  });

  let mobileMenu = $state<HTMLDetailsElement | undefined>(undefined);

  const links = $derived(
    [...GENERAL_LINKS, ...($page.data.user ? LOGGED_IN_LINKS : [{ label: "login", link: `/auth/login?redirectTo=${$page.url.pathname}` }])].map(
      ({ link, ...rest }) => ({
        link,
        ...rest,
        isActive: isLinkActive(link, $page.url.pathname),
      })
    )
  );

  $effect(() => {
    $page.url.pathname;
    if (mobileMenu) {
      mobileMenu.open = false;
    }
  });
</script>

<header
  bind:offsetHeight={height}
  id="sidebar-wrapper"
  class={clsx(
    "z-10 max-h-dvh w-full md:w-64 md:shrink-0 md:self-start xl:w-80 print:hidden",
    "fixed bottom-0 transform-gpu transition-all duration-150 ease-out md:sticky md:top-0",
    "bg-wood-hor dark:bg-dark-wood-hor flex flex-col p-2 md:px-4 md:![background-image:initial]",
    "dark:bg-wood-915 bg-wood-400 md:bg-transparent dark:md:bg-transparent",
    "border-border-light dark:border-border-dark border-b md:border-b-0"
  )}
>
  <nav class="transition-colors-opacity z-[50] flex h-fit w-full items-center gap-[5px] duration-100 md:flex-col md:items-start">
    <div class="flex items-center gap-2 md:block md:items-start md:py-2">
      <a class="shrink-0 select-none px-3 py-2 font-mono text-base" href="/" aria-label="Go to home">blisk</a>
    </div>
    <div class="w-fit min-w-0 grow basis-0 overflow-x-auto pr-2 md:order-last md:w-full md:flex-1 md:pr-0">
      <ul class="ml-auto flex w-max items-center gap-[5px] md:max-h-[50dvh] md:w-full md:grow md:flex-col md:items-start md:overflow-y-auto">
        {#each links as { label, link, isActive }}
          <li class="w-max md:w-full">
            <NavLink href={link} textCenter={false} {isActive}>
              {label}
            </NavLink>
          </li>
        {/each}
        <!-- <li class="w-max md:w-full">
          <NavLink href="/search" textCenter={false} isActive={isLinkActive("/books", $page.url.pathname)}>
            search
          </NavLink>
        </li> -->
      </ul>
    </div>
    <div class="flex flex-row-reverse items-center gap-[5px] md:flex-row">
      <NavUsermenu leftSided />
      <NavToggleScheme />
    </div>
    <SearchBar />
  </nav>
</header>
