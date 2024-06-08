<script lang="ts">
  import GuestLayout from "$components/GuestLayout.svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { signIn } from "@auth/sveltekit/client";
  import { invoke } from "@tauri-apps/api/tauri";
  import { goto } from "$lib/utils";
  import type { User } from "$lib/db";
  import toast from "svelte-french-toast";

  let email: string;
  let password: string;
  let loaded: boolean = false;

  const submit = async (event: Event) => {
    event.preventDefault();
    console.log(email, password);
    // NOTE: you leave me no choice, will this the stupid way xD
    const actual_auth_res: User | null = await invoke("authenticate", {
      email,
      password,
    });
    // INFO: when authentication fails, the response will be null
    if (!actual_auth_res) {
      toast.error("The credentials provided are not valid. Please try again");
      return;
    }
    console.log("[RS_AUTH]:", actual_auth_res);
    toast.success(`Welcome, ${actual_auth_res?.name}!`);
    const hack_auth_res = await signIn("credentials", {
      user: JSON.stringify(actual_auth_res),
    }).catch((e) => {
      console.log("signIn rejected the promise with:", e);
    });
    console.log("[AUTH]: ", hack_auth_res);
  };

  onMount(async () => {
    if ($page.data.session) {
      // TODO: redirect user properly
      await toast.promise(goto("/dashboard"), {
        loading:
          "You are already signed in, redirecting you to the dashboard...",
        success: "Welcome to Project Rapport",
        error: "Failed to redirect, please reload the page.",
      });
    }
    loaded = true;
  });
</script>

<GuestLayout>
  <div class="flex flex-col min-h-screen justify-center items-center">
    {#if loaded}
      {#if !$page.data.session}
        <div class="card card-bordered w-96 bg-base-200 shadow-xl">
          <div class="card-body">
            <h2 class="card-title text-2xl">Authorized Access Only!</h2>
            <p class="text-sm">Please verify your identity...</p>
            <form id="login_form" on:submit={submit}>
              <label class="input input-bordered flex items-center gap-2">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 16 16"
                  fill="currentColor"
                  class="w-4 h-4 opacity-70"
                  ><path
                    d="M2.5 3A1.5 1.5 0 0 0 1 4.5v.793c.026.009.051.02.076.032L7.674 8.51c.206.1.446.1.652 0l6.598-3.185A.755.755 0 0 1 15 5.293V4.5A1.5 1.5 0 0 0 13.5 3h-11Z"
                  /><path
                    d="M15 6.954 8.978 9.86a2.25 2.25 0 0 1-1.956 0L1 6.954V11.5A1.5 1.5 0 0 0 2.5 13h11a1.5 1.5 0 0 0 1.5-1.5V6.954Z"
                  /></svg
                >
                <input
                  type="email"
                  class="grow"
                  placeholder="Email"
                  bind:value={email}
                  required
                />
              </label>

              <label class="input input-bordered flex items-center gap-2 mt-2">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 16 16"
                  fill="currentColor"
                  class="w-4 h-4 opacity-70"
                  ><path
                    fill-rule="evenodd"
                    d="M14 6a4 4 0 0 1-4.899 3.899l-1.955 1.955a.5.5 0 0 1-.353.146H5v1.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-2.293a.5.5 0 0 1 .146-.353l3.955-3.955A4 4 0 1 1 14 6Zm-4-2a.75.75 0 0 0 0 1.5.5.5 0 0 1 .5.5.75.75 0 0 0 1.5 0 2 2 0 0 0-2-2Z"
                    clip-rule="evenodd"
                  /></svg
                >
                <input
                  type="password"
                  class="grow"
                  placeholder="Password"
                  bind:value={password}
                  required
                />
              </label>
            </form>
            <div class="card-actions justify-end">
              <button form="login_form" class="btn btn-primary" type="submit">
                Login
              </button>
            </div>
          </div>
        </div>
      {/if}
    {:else}
      <span class="loading loading-bars loading-lg"></span>
    {/if}
  </div>
</GuestLayout>
