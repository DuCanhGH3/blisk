<script lang="ts">
  const { data } = $props();
</script>

<h2 class="sr-only">Read</h2>
{#each data.volumes as volume}
  <section>
    <h3 class="h5 pb-3">{volume.name}</h3>
    {#each volume.entries as entry}
      <article class="box sm">
        {entry.name}
      </article>
    {/each}
  </section>
{/each}
