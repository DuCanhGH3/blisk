<script>
  import Svg from "$components/Svg.svelte";
</script>

<div class="dark:bg-neutral-915 border-border-light dark:border-border-dark flex flex-row gap-2 rounded-full border bg-white p-1 shadow-md">
  <button>
    <Svg width="36" height="36">
      <title>Like</title>
      <use href="#like-emoji" />
    </Svg>
  </button>
  <button>
    <Svg width="36" height="36">
      <title>Love</title>
      <use href="#heart-emoji" />
    </Svg>
  </button>
  <button>
    <Svg width="36" height="36">
      <title>Haha</title>
      <use href="#haha-emoji" />
    </Svg>
  </button>
</div>
