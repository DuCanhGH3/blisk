<script lang="ts">
  import { enhance } from "$app/forms";
  import LogOut from "$components/icons/LogOut.svelte";
  import UserCircle from "$components/icons/UserCircle.svelte";
  import { clsx } from "$lib/clsx";
  import type { User } from "$lib/types";

  const { user }: { user: Pick<User, "name"> | null } = $props();

  const NAV_USERMENU_CLASS = clsx(
    "group flex w-full items-center rounded-md p-2 text-sm transition-colors duration-100",
    "hover:bg-gray-200 text-black hover:dark:bg-neutral-800 dark:text-white"
  );
</script>

{#if user}
  <details class="details-anim relative inline-block text-left">
    <summary
      class={clsx(
        "flex w-full cursor-pointer flex-row justify-between rounded-md text-base transition-colors-opacity duration-100 md:text-sm",
        "px-3 py-2 font-medium text-black hover:bg-neutral-250 dark:text-white dark:hover:bg-neutral-800"
      )}
      id="navbar-usermenu-button"
    >
      <!-- <img alt="Your avatar" width={32} height={32} class="h-8 w-8 rounded-full text-transparent" src={user.avatar} /> -->
      <p class="line-clamp-1 break-words">
        {user.name}
      </p>
    </summary>
    <div
      class={clsx(
        "absolute right-0 mt-2 w-52 origin-top-right rounded-[10px] shadow-lg transition ease-in-out [&>*]:p-1",
        "border-[0.25px] border-border-light bg-white dark:border-gray-700 dark:bg-black",
        "divide-y divide-neutral-300 ring-1 ring-black/5 focus:outline-none dark:divide-gray-700"
      )}
      aria-labelledby="navbar-usermenu-button"
      id="navbar-usermenu"
      role="menu"
      tabindex={0}
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
        <a href="/" class={NAV_USERMENU_CLASS}>
          <UserCircle width={24} height={24} class="mr-2 h-5 w-5" />
          Your profile
        </a>
        <form method="POST" action="/login?/logout" use:enhance>
          <button class={NAV_USERMENU_CLASS}>
            <LogOut width={24} height={24} class="mr-2 h-5 w-5" />
            Sign out
          </button>
        </form>
      </div>
    </div>
  </details>
{/if}
