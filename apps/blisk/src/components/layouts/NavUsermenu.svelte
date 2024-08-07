<script lang="ts">
  import { enhance } from "$app/forms";
  import { page } from "$app/stores";
  import LogOut from "$components/icons/LogOut.svelte";
  import UserCircle from "$components/icons/UserCircle.svelte";
  import Menu from "$components/Menu.svelte";
  import MenuItem from "$components/MenuItem.svelte";
  import { clsx } from "$lib/clsx";
  import type { User } from "$lib/types";

  interface NavUsermenuProps {
    user: Pick<User, "name"> | null;
    leftSided?: boolean;
    summaryIcon?: boolean;
  }

  const { user, leftSided = false }: NavUsermenuProps = $props();

  let navUsermenu = $state<HTMLDetailsElement | null>(null);

  $effect(() => {
    $page.url.pathname;
    if (navUsermenu) {
      navUsermenu.open = false;
    }
  });
</script>

{#if user}
  <details bind:this={navUsermenu} id="nav-usermenu" class="details-anim relative inline-block text-left">
    <summary class="nav-button flex-shrink-0" id="navbar-usermenu-button">
      <img alt="Your avatar" width={24} height={24} class="h-6 w-6 rounded-full text-transparent" src="/no-avatar.webp" />
    </summary>
    <Menu
      class={clsx("w-52 origin-top-right bg-white dark:bg-black", leftSided ? "right-0 md:left-0" : "right-0")}
      id="navbar-usermenu"
      aria-labelledby="navbar-usermenu-button"
    >
      <div>
        <div class="flex flex-row items-center gap-2 p-2" id="navbar-usermenu-items" role="menuitem" tabindex={-1}>
          <!-- <img alt="Your avatar" width={40} height={40} class="h-10 w-10 rounded-full text-transparent" src={user.avatar} /> -->
          <div>
            <div class="text-base font-bold">
              {user.name}
            </div>
          </div>
        </div>
      </div>
      <div>
        <MenuItem as="a" href="/users/{user.name}">
          <UserCircle width={20} height={20} class="mr-2 h-auto w-5" aria-hidden="true" tabindex={-1} />
          Your profile
        </MenuItem>
        <form method="POST" action="/login?/logout" use:enhance>
          <MenuItem as="button">
            <LogOut width={20} height={20} class="mr-2 h-auto w-5" aria-hidden="true" tabindex={-1} />
            Sign out
          </MenuItem>
        </form>
      </div>
    </Menu>
  </details>
{/if}
