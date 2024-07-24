<script>
  import { page } from "$app/stores";
</script>

<div class="flex h-full w-full items-center justify-center">
  <div class="flex w-[90dvw] max-w-[500px] flex-col justify-center gap-6 rounded-md p-10">
    <h1>
      <span class="sr-only">An error occurred - </span>{$page.status}
    </h1>
    <h2>{$page.error?.message}</h2>
    <button class="button filled" onclick={() => window.location.reload()}>Refresh</button>
  </div>
</div>
