<script>
  import HeartFilled from "$components/icons/HeartFilled.svelte";
  import ThumbUpFilled from "$components/icons/ThumbUpFilled.svelte";
</script>

<div class="dark:bg-neutral-915 border-border-light dark:border-border-dark flex flex-row gap-2 rounded-full border bg-white p-1 shadow-md">
  <button class="react-button like">
    <ThumbUpFilled width={20} height={20} fill="#ffffff" />
  </button>
  <button class="react-button heart">
    <HeartFilled width={20} height={20} fill="#ffffff" />
  </button>
  <button class="react-button laugh">
    <span class="laugh-emoji">
      <span class="eyes-cont"> </span>
      <span class="mouth"></span>
    </span>
  </button>
</div>
