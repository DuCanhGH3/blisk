<script lang="ts">
  const { data } = $props();
</script>

<div>
  {data.post.title}
  {data.post.content}
  {data.post.author_name}
</div>
