<script lang="ts" generics="T extends { id: number | string }">
  import { browser } from "$app/environment";
  import type { Snippet } from "svelte";

  interface VirtualScrollerProps {
    items: T[];
    renderer: Snippet<[T]>;
  }

  const { items, renderer }: VirtualScrollerProps = $props();

  class HeightTree {
    private n = $state(0);
    private bit = $state<number[]>([]);

    constructor(arrOrLength: number | number[]) {
      this.n = typeof arrOrLength === "number" ? arrOrLength : arrOrLength.length;
      this.bit = Array.from({ length: this.n }, () => 0);
      if (Array.isArray(arrOrLength)) {
        for (let i = 0; i < this.n; ++i) {
          this.bit[i] += arrOrLength[i];
          const r = i | (i + 1);
          if (r < this.n) {
            this.bit[r] += this.bit[i];
          }
        }
      }
    }

    rangeSum(l: number, r: number) {
      return this.sum(r) - (l > 0 ? this.sum(l - 1) : 0);
    }

    sum(r: number) {
      let ret = 0;
      for (; r >= 0; r = (r & (r + 1)) - 1) {
        ret += this.bit[r];
      }
      return ret;
    }

    add(idx: number, delta: number) {
      for (; idx < this.n; idx = idx | (idx + 1)) {
        this.bit[idx] += delta;
      }
    }
  }

  let container = $state<HTMLDivElement | null>(null);
  // TODO: add a function that calculates `start` and `end`,
  // which should also be used when `items` is updated. 
  let start = $state(0);
  let end = $state(items.length - 1);
  const rows = $state<(HTMLDivElement | null)[]>([]);
  let height = $state<number[]>(Array.from({ length: items.length }, () => 0));
  let heightTree = $state<HeightTree>(new HeightTree(height));

  const visible = $derived(items.slice(start, end + 1));

  const handleScroll = () => {
    if (!container) return;
    for (let i = 0; i <= visible.length; ++i) {
      const row = rows[i];
      if (row) {
        let offset = row.offsetHeight;
        if (offset !== height[start + i]) {
          heightTree.add(i, offset - height[start + i]);
          height[start + i] = offset;
        }
      }
    }
    let tl = 0;
    let tr = items.length - 1;
    let containerTop = container.getBoundingClientRect().y;
    if (containerTop >= 0) {
      // Container's top is in view, start from the first item.
      start = 0;
    } else {
      // Otherwise, we need to find the height of all the prefix
      // items that are out of view.
      containerTop = Math.abs(containerTop);

      while (tl < tr) {
        const tm = tl + ((tr - tl) >> 1);
        if (heightTree.sum(tm) < containerTop) {
          tl = tm + 1;
        } else {
          tr = tm;
        }
      }

      start = tl;
      tr = items.length - 1;
    }
    // When we render from the start, this will cause the algorithm to
    // render an extra of `containerTop` pixels, but that's acceptable.
    const totalRenderingHeight = containerTop + window.innerHeight;
    while (tl < tr) {
      let tm = tl + ((tr - tl) >> 1);
      if (heightTree.rangeSum(start, tm) <= totalRenderingHeight) {
        tl = tm + 1;
      } else {
        tr = tm;
      }
    }
    end = tr;
  };
</script>

<svelte:window onscroll={handleScroll} />

{#if browser}
  {@const paddingTop = start > 0 ? heightTree.sum(start - 1) : 0}
  {@const paddingBottom = end < items.length - 1 ? heightTree.rangeSum(end + 1, items.length - 1) : 0}
  <div bind:this={container} style="padding-top:{paddingTop}px;padding-bottom:{paddingBottom}px">
    {#each visible as item, i (item.id)}
      <div bind:this={rows[i]}>
        {@render renderer(item)}
      </div>
    {/each}
  </div>
{:else}
  <div>
    {#each items as item}
      {@render renderer(item)}
    {/each}
  </div>
{/if}
