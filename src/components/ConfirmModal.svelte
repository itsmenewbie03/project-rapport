<script lang="ts">
  import toast from "svelte-french-toast";
  import Icon from "./Icon.svelte";
  import { goto } from "$lib/utils";
  import { invoke } from "@tauri-apps/api/tauri";
  import { page } from "$app/stores";

  let handle: HTMLDialogElement;

  const hide = () => {
    handle.classList.remove("modal-open");
  };

  export const show = () => {
    handle.classList.add("modal-open");
  };

  let password: string = "";

  const _confirm = async (event: Event) => {
    // TEST: we will implement the actual password verification tomorrow
    event.preventDefault();
    const is_verified = await invoke("authenticate", {
      email: $page.data.session.email,
      password: password,
    });
    if (is_verified) {
      localStorage.setItem("allowed_to_exit", "true");
      hide();
      await toast.promise(goto("/dashboard"), {
        loading: "Access granted!.",
        success: "Welcome Back!",
        error: "Failed to redirect, please reload the page.",
      });
      // NOTE: we will force stop recording on exit
      await invoke("clear_recording", {
        id: "I DON'T HAVE ENOUGH TIME TO IMPLEMENT THIS PROPERLY xD!",
      });
      localStorage.removeItem("allowed_to_exit");
    } else {
      hide();
      toast.error("Incorrect password. Please try again.");
      password = "";
    }
  };
</script>

<dialog id="confirm_model" class="modal" bind:this={handle}>
  <div class="modal-box">
    <h3 class="font-bold text-lg">Authorized Access Only!</h3>
    <p class="py-4">Please verify you indentity by entering your password.</p>
    <form class="form-control" id="password_confirm_form" on:submit={_confirm}>
      <label class="input input-bordered flex items-center gap-2">
        <Icon name="password" _class="size-4 opacity-70" viewBox="0 0 16 16" />
        <input type="password" class="grow" bind:value={password} required />
      </label>
    </form>
    <div class="modal-action">
      <button class="btn btn-primary" type="submit" form="password_confirm_form"
        >Confirm</button
      >
      <button class="btn" on:click={hide}>Cancel</button>
    </div>
  </div>
</dialog>
