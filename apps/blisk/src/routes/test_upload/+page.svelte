<script>
  import { PUBLIC_BACKEND_URL } from "$env/static/public";
</script>

<form action="{PUBLIC_BACKEND_URL}/assets/upload" method="POST" enctype="multipart/form-data">
  <input type="file" name="files" multiple required />
  <button type="submit">Submit</button>
</form>