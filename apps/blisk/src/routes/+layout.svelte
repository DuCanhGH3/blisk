<script lang="ts">
  import "../app.css";

  import { onNavigate } from "$app/navigation";

  import { getSerwist } from "virtual:serwist";

  import { dev } from "$app/environment";
  import { page } from "$app/stores";
  import { isColorScheme } from "$lib/isColorScheme";
  import { colorScheme } from "$lib/stores/colorScheme";
  import { PUBLIC_CANONICAL_URL } from "$env/static/public";
  import VerticalNavbar from "$components/layouts/VerticalNavbar.svelte";

  const { data, children } = $props();
  const isDark = $derived($colorScheme === "dark");
  const title = $derived($page.data.title ? `${$page.data.title} - blisk` : "blisk");
  const ogImage = $derived($page.data.ogImage);

  $effect(() => {
    const registerSerwist = async () => {
      if (!dev && "serviceWorker" in navigator) {
        const serwist = await getSerwist();
        serwist?.addEventListener("installed", () => {
          console.log("Serwist installed!");
        });
        void serwist?.register();
      }
    };
    registerSerwist();
  });

  $effect(() => {
    const newTheme = document.documentElement.dataset.theme;
    $colorScheme = isColorScheme(newTheme) ? newTheme : "light";
    colorScheme.subscribe((value) => {
      document.documentElement.dataset.theme = value;
      localStorage.setItem("theme", value);
    });
  });

  onNavigate((navigation) => {
    if (!("startViewTransition" in document)) return;

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });
</script>

<svelte:head>
  <title>{title}</title>
  <link rel="canonical" href={new URL($page.url.pathname, PUBLIC_CANONICAL_URL).href} />
  <!-- <link rel="manifest" href="/manifest.webmanifest" /> -->
  <meta property="og:title" content={title} />
  {#if ogImage}
    <meta property="og:image" content={ogImage} />
  {/if}
  <meta name="twitter:title" content={title} />
  <meta name="theme-color" content={isDark ? "#000000" : "#FFFFFF"} />
</svelte:head>

<a class="absolute -top-full z-[100] text-black underline focus:top-0 dark:text-white" href="#main-content">Skip to main content</a>
<div class="flex flex-1 flex-col md:flex-row">
  <!-- <VerticalNavbar user={data.user} /> -->
  <main id="main-content">
    {@render children()}
  </main>
</div>
