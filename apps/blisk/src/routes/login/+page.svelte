<script>
  import { enhance } from "$app/forms";
  import Input from "$components/Input.svelte";

  const { form } = $props();
  let isLoading = $state(false);
</script>

<div class="flex w-full items-center justify-center self-stretch p-4">
  <div class="container flex w-[90dvw] max-w-[500px] items-center gap-6 rounded-lg p-8 shadow-xl">
    <form
      method="POST"
      action="?/login"
      class="flex w-full flex-col gap-3"
      use:enhance={() => {
        isLoading = true;
        return async ({ update }) => {
          isLoading = false;
          await update();
        };
      }}
    >
      <h1>Login</h1>
      <Input
        id="login-username-input"
        label="Username"
        name="username"
        type="text"
        errorText={form?.validationError?.username}
        errorTextId="login-username-error-text"
      />
      <Input
        id="login-password-input"
        label="Password"
        name="password"
        type="password"
        errorText={form?.validationError?.password}
        errorTextId="login-password-error-text"
      />
      <button class="button filled" disabled={isLoading} type="submit">Login</button>
      <a class="link" href="/register">New? Register a new account!</a>
      {#if form?.error}
        <p class="text-error">{form.error}</p>
      {/if}
    </form>
  </div>
</div>
