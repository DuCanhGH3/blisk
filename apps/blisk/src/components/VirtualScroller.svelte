<script lang="ts" generics="T extends { id: number | string }">
  import type { Ref } from "$lib/types";
  import { range } from "$lib/utils";
  // Note: since the virtual scroller destroys any component not in view,
  // states will not work properly.
  import { tick, type Snippet } from "svelte";

  interface VirtualScrollerProps {
    items: T[];
    /**
     * The data renderer. Should be stateless to work properly.
     * We pass `Ref<T>` instead of `T` so that binding is possible,
     * allowing the component to assign its states to the data.
     */
    renderer: Snippet<[Ref<T>]>;
  }

  const { items = $bindable(), renderer }: VirtualScrollerProps = $props();

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
      return this.sum(r) - this.sum(l - 1);
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
  // During SSR, render the whole thing.
  let start = $state(0);
  let end = $state(items.length - 1);
  const rows = $state<(HTMLDivElement | null)[]>([]);
  let height = $state<number[]>(null!);
  let heightTree = $state<HeightTree | null>(null);

  const paddingTop = $derived(start > 0 && heightTree ? heightTree.sum(start - 1) : 0);
  const paddingBottom = $derived(end < items.length - 1 && heightTree ? heightTree.rangeSum(end + 1, items.length - 1) : 0);

  /**
   * The bulk of the virtual scroller. Requires that all elements' heights have been calculated.
   * The way we do that is to initially render the whole thing, create a height map, then run this
   * function. 
   */
  const recalculate = () => {
    if (!container || !heightTree) return;
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

  const refresh = async () => {
    start = 0;
    end = items.length - 1;
    await tick();
    heightTree = null;
    height = Array.from({ length: items.length }, () => 0);
    for (let i = start; i <= end; ++i) {
      const row = rows[i - start];
      if (row) {
        height[i] = row.offsetHeight;
      }
    }
    heightTree = new HeightTree(height);
    recalculate();
  };

  const handleLayout = () => {
    if (!heightTree) return;
    for (let i = 0; i <= end - start + 1; ++i) {
      const row = rows[i];
      if (row) {
        let offset = row.offsetHeight;
        if (offset !== height[start + i]) {
          heightTree.add(start + i, offset - height[start + i]);
          height[start + i] = offset;
        }
      }
    }
    recalculate();
  };

  $effect(() => {
    refresh();
  });
</script>

<svelte:window onscroll={handleLayout} onresize={handleLayout} />

<div bind:this={container} style="padding-top:{paddingTop}px;padding-bottom:{paddingBottom}px">
  {#each range(start, end, (i) => items[i]) as item, i (item.id)}
    <div bind:this={rows[i]}>
      {@render renderer({ ref: item })}
    </div>
  {/each}
</div>
