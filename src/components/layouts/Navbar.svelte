<script lang="ts">
  import { page } from "$app/stores";
  import { clsx } from "$lib/clsx";
  import { isLinkActive } from "$lib/isLinkActive";
  import type { User } from "$lib/types";

  import { LOGGED_IN_LINKS, LOGGED_OUT_LINKS, GENERAL_LINKS } from "./navbar-constants";
  import NavLink from "./NavLink.svelte";
  import NavToggleScheme from "./NavToggleScheme.svelte";
  import NavUsermenu from "./NavUsermenu.svelte";

  interface NavbarProps {
    user: Pick<User, "name"> | null;
  }

  const { user }: NavbarProps = $props();

  const links = $derived(
    [...GENERAL_LINKS, ...(user ? LOGGED_IN_LINKS : LOGGED_OUT_LINKS)].map(({ link, ...rest }) => ({
      link,
      ...rest,
      isActive: isLinkActive(link, $page.url.pathname),
    }))
  );

  let mobileMenu = $state<HTMLDetailsElement | undefined>(undefined);

  $effect(() => {
    $page.url.pathname;
    if (mobileMenu) {
      mobileMenu.open = false;
    }
  });
</script>

<nav
  class={clsx(
    "sticky top-0 z-[50] h-fit max-h-dvh border-b transition-colors-opacity duration-100",
    "border-neutral-300 bg-white dark:border-neutral-800 dark:bg-black"
  )}
>
  <div class="mx-auto max-w-7xl px-2 py-2 md:px-6 lg:px-8">
    <div class="relative flex items-center justify-between overflow-x-clip">
      <div class="flex items-center gap-2 md:block md:items-start md:py-2">
        <a class="shrink-0 font-mono text-base uppercase tracking-widest" href="/" aria-label="Go to home">PERDRIFT</a>
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
                <li aria-current={isActive ? "page" : undefined}>
                  <NavLink href={link} textCenter={false} {isActive}>
                    {label}
                  </NavLink>
                </li>
              {/each}
            </ul>
          </div>
        </details>
        <div class="hidden h-full grow items-center overflow-x-hidden pr-2 md:ml-6 md:flex md:pr-0">
          <div class="overflow-x-overlay hidden h-full grow flex-row-reverse items-center gap-[5px] overflow-x-auto md:flex">
            <ul class="flex max-h-full flex-row gap-[inherit]">
              {#each links as { label, link, isActive }}
                <li aria-current={isActive ? "page" : undefined}>
                  <NavLink href={link} {isActive}>
                    {label}
                  </NavLink>
                </li>
              {/each}
            </ul>
          </div>
        </div>
        <NavUsermenu {user} />
        <NavToggleScheme />
      </div>
    </div>
  </div>
</nav>
