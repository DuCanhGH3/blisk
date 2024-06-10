<script lang="ts">
  const { data } = $props();
</script>

<article class="flex h-full w-full max-w-6xl flex-col gap-2 p-2 md:py-8">
  <h1 class="order-2">{data.post.title}</h1>
  <h2 class="text-comment order-1 text-xl">{data.post.author_name}</h2>
  <div class="order-3 mt-4">
    {data.post.content}
  </div>
</article>
