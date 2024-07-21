<script lang="ts">
  import TableBody from "./TableBody.svelte";
  import TableHead from "./TableHead.svelte";
  import TableRow from "./TableRow.svelte";
  import TableCell from "./TableCell.svelte";
  import type { RendererProps, Tokens } from "./types";

  const { header, rows }: RendererProps<Tokens.Table> = $props();
</script>

<table>
  <TableHead>
    <TableRow>
      {#each header as headerItem}
        <TableCell {...headerItem} />
      {/each}
    </TableRow>
  </TableHead>
  <TableBody>
    {#each rows as row}
      <TableRow>
        {#each row as cell}
          <TableCell {...cell} />
        {/each}
      </TableRow>
    {/each}
  </TableBody>
</table>
