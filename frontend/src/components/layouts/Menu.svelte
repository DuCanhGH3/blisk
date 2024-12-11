<script lang="ts">
  import { enhance } from "$app/forms";
  import { page } from "$app/stores";
  import LogOut from "$components/icons/LogOut.svelte";
  import UserCircle from "$components/icons/UserCircle.svelte";
  import Menu from "$components/Menu.svelte";
  import MenuItem from "$components/MenuItem.svelte";
  import { getProfilePicture } from "$lib/utils";

  let navUsermenu = $state<HTMLDetailsElement | null>(null);

  const user = $derived($page.data.user);

  $effect(() => {
    $page.url.pathname;
    if (navUsermenu) {
      navUsermenu.open = false;
    }
  });
</script>

{#if user}
  <details bind:this={navUsermenu} id="nav-usermenu" class="relative inline-block text-left">
    <summary class="nav-button shrink-0" id="navbar-usermenu-button">
      <img
        src={getProfilePicture(user.picture)}
        class="h-6 w-6 rounded-full text-transparent"
        width={24}
        height={24}
        loading="lazy"
        decoding="async"
        alt="Your avatar"
      />
    </summary>
    <Menu
      id="navbar-usermenu"
      class="animate-fly-up bottom-full right-0 z-50 w-52 origin-top-right -translate-y-4"
      style="--fly-translate-y-up: 1rem;"
      aria-labelledby="navbar-usermenu-button"
    >
      <div id="navbar-usermenu-items" role="menuitem" tabindex={-1}>
        <div class="flex flex-row items-center gap-2 p-2 text-base font-bold" id="navbar-usermenu-items" role="menuitem" tabindex={-1}>
          {user.name}
        </div>
      </div>
      <div>
        <MenuItem as="a" href="/users/{user.name}">
          <UserCircle width={20} height={20} class="mr-2 h-auto w-5" aria-hidden="true" tabindex={-1} />
          Your profile
        </MenuItem>
        <form method="POST" action="/auth/login?/logout&redirectTo={$page.url.pathname}" use:enhance>
          <MenuItem as="button">
            <LogOut width={20} height={20} class="mr-2 h-auto w-5" aria-hidden="true" tabindex={-1} />
            Sign out
          </MenuItem>
        </form>
      </div>
    </Menu>
  </details>
{/if}
