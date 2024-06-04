<script lang="ts">
  import { page } from "$app/stores";
  import { clsx } from "$lib/clsx";
  import { isLinkActive } from "$lib/isLinkActive";
  import { BREAKPOINTS } from "$lib/constants";
  import type { User } from "$lib/types";

  import NavLink from "./NavLink.svelte";
  import NavToggleScheme from "./NavToggleScheme.svelte";
  import { LOGGED_IN_LINKS, LOGGED_OUT_LINKS, GENERAL_LINKS } from "./navbar-constants";
  import NavUsermenu from "./NavUsermenu.svelte";

  interface NavbarProps {
    user: Pick<User, "name"> | null;
  }

  const { user }: NavbarProps = $props();

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
    [...GENERAL_LINKS, ...(user ? LOGGED_IN_LINKS : LOGGED_OUT_LINKS)].map(({ link, ...rest }) => ({
      link,
      ...rest,
      isActive: isLinkActive(link, $page.url.pathname),
    }))
  );

  $effect(() => {
    $page.url.pathname;
    if (mobileMenu) {
      mobileMenu.open = false;
    }
  });
</script>

<div
  id="sidebar-wrapper"
  class={clsx(
    "z-10 max-h-dvh w-full md:w-64 md:shrink-0 md:self-start xl:w-80 print:hidden",
    "sticky top-0 transform-gpu transition-all duration-150 ease-out",
    "flex flex-col bg-white p-2 dark:bg-black md:bg-transparent md:px-4 dark:md:bg-transparent",
    "border-b border-neutral-300 dark:border-neutral-800 md:border-b-0"
  )}
>
  <nav class="z-[50] h-fit transition-colors-opacity duration-100">
    <div class="relative mx-auto flex flex-row justify-between overflow-x-clip md:flex-col">
      <div class="flex items-center gap-2 md:block md:items-start md:py-2">
        <a class="shrink-0 select-none px-3 py-2 font-mono text-base" href="/" aria-label="Go to home">blisk</a>
      </div>
      <div class="flex flex-col gap-[5px] md:flex-col-reverse">
        <div class="hidden h-full pr-2 md:flex md:pr-0">
          <div class="overflow-x-overlay hidden h-full grow flex-row gap-[5px] overflow-x-auto md:flex">
            <ul class="flex max-h-[50dvh] w-full flex-col gap-[inherit] overflow-y-auto">
              {#each links as { label, link, isActive }}
                <li class="w-full">
                  <NavLink href={link} textCenter={false} {isActive}>
                    {label}
                  </NavLink>
                </li>
              {/each}
            </ul>
          </div>
        </div>
        <div class="flex flex-row-reverse items-center gap-[5px] md:flex-row">
          <details bind:this={mobileMenu} class="details-anim relative ml-3 md:hidden" id="nav-mobile-menu">
            <summary
              class={clsx(
                "flex h-[2rem] w-[2rem] cursor-pointer flex-col justify-center gap-[0.5rem]",
                "[&>span]:bg-black [&>span]:transition-all [&>span]:dark:bg-white",
                "[&>span]:h-[0.2rem] [&>span]:w-full [&>span]:rounded-md"
              )}
              aria-label="Toggle navbar menu"
            >
              <span class="origin-center duration-300"></span>
              <span class="duration-200 ease-out"></span>
              <span class="origin-center duration-300"></span>
            </summary>
            <div class="absolute right-0 w-[150px] md:hidden">
              <ul
                class="relative top-2 max-h-[60dvh] space-y-1 overflow-y-auto rounded-[14px] border border-neutral-300 bg-white p-2 dark:border-neutral-800 dark:bg-black"
              >
                {#each links as { label, link, isActive }}
                  <li>
                    <NavLink href={link} textCenter={false} {isActive}>
                      {label}
                    </NavLink>
                  </li>
                {/each}
              </ul>
            </div>
          </details>
          <NavUsermenu {user} leftSided summaryIcon />
          <NavToggleScheme />
        </div>
      </div>
    </div>
  </nav>
</div>
