<script lang="ts">
  import TableCell from "./TableCell.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { header, rows }: RendererProps<Tokens.Table> = $props();
</script>

<div class="w-full overflow-x-auto">
  <table
    class="[&_:is(td,th)]:border-border-light dark:[&_:is(td,th)]:border-border-dark table-auto border-collapse border-spacing-4 [&_:is(td,th)]:border"
  >
    <thead>
      <tr>
        {#each header as headerItem}
          <TableCell {...headerItem} />
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each rows as row}
        <tr>
          {#each row as cell}
            <TableCell {...cell} />
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
