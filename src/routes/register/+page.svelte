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
      action="?/register"
      class="flex w-full flex-col gap-3"
      use:enhance={() => {
        isLoading = true;
        return async ({ update }) => {
          isLoading = false;
          await update();
        };
      }}
    >
      <h1>Register</h1>
      <Input
        id="register-username-input"
        label="Username"
        name="username"
        type="text"
        errorText={form?.usernameError}
        errorTextId="register-username-error-text"
      />
      <Input id="register-email-input" label="Email" name="email" type="text" errorText={form?.emailError} errorTextId="register-email-error-text" />
      <Input
        id="register-password-input"
        label="Password"
        name="password"
        type="password"
        errorText={form?.passwordError}
        errorTextId="register-password-error-text"
      />
      <button class="button filled" disabled={isLoading} type="submit">Register</button>
      <a class="link" href="/login">Already have an account?</a>
      {#if form?.error}
        <p class="text-error">{form.error}</p>
      {/if}
    </form>
  </div>
</div>
