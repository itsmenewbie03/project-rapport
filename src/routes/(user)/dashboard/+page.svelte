<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import toast from "svelte-french-toast";
  let loaded: boolean = false;
  onMount(async () => {
    if (!$page.data.session) {
      toast.error("Please login first.");
      await goto("/login");
    }
    // TEST: for aesthetics we will delay the load for a second xD
    setTimeout(() => {
      loaded = true;
    }, 1000);
  });
</script>

<UserLayout>
  <div class="flex flex-col h-[calc(100vh-116px)] justify-center items-center">
    {#if loaded}
      {#if $page.data.session}
        <h1 class="text-4xl">Dashboard Page</h1>
        <p class="text-md">
          This is a protected content. You can access this content because you
          are signed in.
        </p>
        <button
          on:click={(_e) => {
            alert("NOT IMPLEMENTED");
          }}
          class="btn btn-primary"
        >
          LOGOUT!
        </button>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <span class="loading loading-bars loading-lg"></span>
    {/if}
  </div>
</UserLayout>
