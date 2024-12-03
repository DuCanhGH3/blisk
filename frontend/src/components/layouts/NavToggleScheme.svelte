<script lang="ts">
  import MoonStars from "$components/icons/MoonStars.svelte";
  import Sun from "$components/icons/Sun.svelte";
  import { svgIconAttrs } from "$components/renderers/renderer-constants";
  import { toggleColorScheme } from "$lib/stores/colorScheme";
</script>

<button class="nav-button flex-shrink-0" onclick={() => toggleColorScheme()}>
  <span class="visible-dark">
    <MoonStars {...svgIconAttrs} />
  </span>
  <span class="visible-light">
    <Sun {...svgIconAttrs} />
  </span>
  <span class="sr-only">Toggle color scheme</span>
</button>
