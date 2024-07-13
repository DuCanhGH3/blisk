<script>
  import Svg from "$components/Svg.svelte";
</script>

<div class="dark:bg-neutral-915 border-border-light dark:border-border-dark flex flex-row gap-2 rounded-full border bg-white p-1 shadow-md">
  <button>
    <Svg width="28" height="28">
      <use href="#like-emoji" />
    </Svg>
  </button>
  <button>
    <Svg width="28" height="28">
      <use href="#heart-emoji" />
    </Svg>
  </button>
  <button>
    <Svg width="28" height="28">
      <use href="#haha-emoji" />
    </Svg>
  </button>
</div>
