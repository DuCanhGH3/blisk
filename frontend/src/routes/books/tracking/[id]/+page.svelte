<script lang="ts">
  import { getImage } from "$lib/utils";

  const { data } = $props();
</script>

<div class="mx-auto flex h-full w-full flex-col gap-8 p-4 md:px-20 md:py-10">
  <div class="flex w-full flex-col gap-4 lg:flex-row lg:gap-8">
    <img
      src={getImage(data.data.book_cover, "/test-cover.jpg")}
      width="192"
      height="288"
      alt=""
      class="h-72 w-48 select-none shadow-[5px_2px_20px_-1px_#000000]"
    />
    <h1 class="pt-4 text-5xl lg:text-8xl">{data.data.book_title}</h1>
  </div>
  <div>
    {data.data.starts_at}
    {data.data.ends_at}
  </div>
</div>
