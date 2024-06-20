<script lang="ts">
  const { data } = $props();
</script>

<div>
  {#each data.books as { title, summary, reviews }}
    <div>
      <p>{title}</p>
      <p>{summary}</p>
      {#each reviews as { title, content, author_name, reaction }}
        <div>
          {title}
          {content}
          {author_name}
          {reaction}
        </div>
      {/each}
    </div>
  {/each}
</div>
