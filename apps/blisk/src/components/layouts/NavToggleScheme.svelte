<script lang="ts">
  import MoonStars from "$components/icons/MoonStars.svelte";
  import Sun from "$components/icons/Sun.svelte";
  import { toggleColorScheme } from "$lib/stores/colorScheme";
</script>

<button class="nav-button flex-shrink-0" onclick={() => toggleColorScheme()}>
  <span class="visible-dark">
    <MoonStars width={24} height={24} class="min-w-6 max-w-6 min-h-6 max-h-6" />
  </span>
  <span class="visible-light">
    <Sun width={24} height={24} class="min-w-6 max-w-6 min-h-6 max-h-6" />
  </span>
  <span class="sr-only">Toggle color scheme</span>
</button>
