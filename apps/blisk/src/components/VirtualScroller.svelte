<script lang="ts" generics="T extends { id: number | string }">
  // Note: since the virtual scroller destroys any component not in view,
  // states will not work properly.
  import { tick, untrack, type Snippet } from "svelte";
  import type { Ref } from "$lib/types";
  import { range } from "$lib/utils";
  import ProgressRing from "./ProgressRing.svelte";

  interface VirtualScrollerProps {
    items: T[];
    /**
     * The data renderer. Should be stateless to work properly.
     * We pass `Ref<T>` instead of `T` so that binding is possible,
     * allowing the component to assign its states to the data.
     */
    renderer: Snippet<[Ref<T>]>;
    /**
     * Replaces the default loading spinner.
     */
    loading?: Snippet<[]>;
    /**
     * Replaces the default error renderer.
     */
    error?: Snippet<[]>;
    /**
     * A function that should load more data. If not defined, the
     * virtual scroller does not implement infinite loading.
     */
    loadMore?(): Promise<T[]> | T[];
  }

  const { items = $bindable(), renderer, loading, error, loadMore }: VirtualScrollerProps = $props();

  class HeightTree {
    private n = $state(0);
    private bit = $state<number[]>([]);

    constructor(arrOrLength: number | number[]) {
      this.n = typeof arrOrLength === "number" ? arrOrLength : arrOrLength.length;
      this.bit = Array.from({ length: this.n }, () => 0);
      if (Array.isArray(arrOrLength)) {
        for (let i = this.n - 1; i >= 0; --i) {
          this.bit[i] += arrOrLength[i];
          const r = (i & (i + 1)) - 1;
          if (r >= 0) {
            this.bit[r] += this.bit[i];
          }
        }
      }
    }

    /**
     * Get sum of [l, r].
     * @param l
     * @param r
     */
    rangeSum(l: number, r: number) {
      return this.sum(l) - this.sum(r + 1);
    }

    /**
     * Get sum of [r, n].
     * @param r
     */
    sum(r: number) {
      let ret = 0;
      for (; r < this.n; r = r | (r + 1)) {
        if (!this.bit[r]) this.bit[r] = 0;
        ret += this.bit[r];
      }
      return ret;
    }

    /**
     * Increase `arr[idx]` by `delta`.
     * @param idx
     * @param delta
     */
    add(idx: number, delta: number) {
      this.n = Math.max(idx + 1, this.n);
      for (; idx >= 0; idx = (idx & (idx + 1)) - 1) {
        if (!this.bit[idx]) this.bit[idx] = 0;
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
  let hasReachedEnd = $state(false);
  let errorMessage = $state<string | null>(null);

  let isLoadingMore = false;

  const paddingTop = $derived(start > 0 && heightTree ? heightTree.rangeSum(0, start - 1) : 0);
  const paddingBottom = $derived(end < items.length - 1 && heightTree ? heightTree.sum(end + 1) : 0);

  const updateTree = () => {
    if (!heightTree) return;
    for (let i = start; i <= end; ++i) {
      const row = rows[i - start];
      if (row) {
        let offset = row.offsetHeight;
        if (offset !== height[i]) {
          if (!height[i]) height[i] = 0;
          heightTree.add(i, offset - height[i]);
          height[i] = offset;
        }
      }
    }
  };

  const loadMoreData = async () => {
    if (container && loadMore && !hasReachedEnd && !isLoadingMore) {
      const containerRect = container.getBoundingClientRect();
      const remaining = containerRect.height - (Math.abs(containerRect.y) + window.innerHeight);
      if (remaining < 100) {
        isLoadingMore = true;
        try {
          errorMessage = null;
          const data = await loadMore();
          if (data.length > 0) {
            items.push(...data);
            end = items.length - 1;
            await tick();
            updateTree();
          } else {
            hasReachedEnd = true;
          }
        } catch (err) {
          if (err instanceof Error) {
            errorMessage = err.message;
          }
        }
        isLoadingMore = false;
      }
    }
  };

  /**
   * The bulk of the virtual scroller. Requires that all elements' heights have been calculated.
   * The way we do that is to initially render the whole thing, create a height map, then run this
   * function.
   */
  const recalculate = () => {
    if (!container || !heightTree) return;
    const containerRect = container.getBoundingClientRect();
    let containerTop = containerRect.y;
    let tl = 0;
    let tr = items.length - 1;
    if (containerTop >= 0) {
      // Container's top is in view, start from the first item.
      start = 0;
    } else {
      // Otherwise, we need to find the height of all the prefix
      // items that are out of view.
      containerTop = Math.abs(containerTop);

      while (tl < tr) {
        const tm = tl + ((tr - tl) >> 1);
        if (heightTree.rangeSum(0, tm) < containerTop) {
          tl = tm + 1;
        } else {
          tr = tm;
        }
      }

      start = tl;
      tr = items.length - 1;
    }
    const totalRenderingHeight = containerTop + window.innerHeight;
    while (tl < tr) {
      let tm = tl + ((tr - tl) >> 1);
      if (heightTree.rangeSum(0, tm) <= totalRenderingHeight) {
        tl = tm + 1;
      } else {
        tr = tm;
      }
    }
    end = tr;
  };

  const handleLayout = async () => {
    if (!heightTree) return;
    await loadMoreData();
    updateTree();
    recalculate();
  };

  $effect(() => {
    // `refresh` will **not** run when `items` updates. In the future, we may expose
    // this function so that other components can recreate the tree themselves.
    const refresh = async () => {
      start = 0;
      end = untrack(() => items.length - 1);
      await tick();
      // In the case the user is loaded at the bottom of the container, try loading more data.
      await loadMoreData();
      heightTree = null;
      const heightTreeLength = end - start + 1;
      height = Array.from({ length: heightTreeLength }, () => 0);
      for (let i = 0; i < heightTreeLength; ++i) {
        const row = rows[i];
        if (row) {
          height[start + i] = row.offsetHeight;
        }
      }
      heightTree = new HeightTree(height);
      recalculate();
    };
    refresh();
  });

  // This listens for external updates.
  $effect(() => {
    items;
    untrack(() => {
      updateTree();
      recalculate();
    });
  });
</script>

<svelte:window onscroll={handleLayout} onresize={handleLayout} />

<div bind:this={container} style="padding-top:{paddingTop}px;padding-bottom:{paddingBottom}px">
  {#each range(start, end, (idx) => items[idx]) as item, i (item?.id)}
    {#if item}
      <div bind:this={rows[i]}>
        {@render renderer({ ref: item })}
      </div>
    {/if}
  {/each}
</div>
{#if !!loadMore && !hasReachedEnd}
  {#if errorMessage}
    {#if error}
      {@render error()}
    {:else}
      <p class="text-error-light dark:text-error-dark">Failed to load more: {errorMessage}</p>
    {/if}
  {:else if loading}
    {@render loading()}
  {:else}
    <div class="flex h-fit flex-row items-center gap-2">
      <ProgressRing size="sm" aria-hidden="true" tabindex={-1} />
      Loading...
    </div>
  {/if}
{/if}
