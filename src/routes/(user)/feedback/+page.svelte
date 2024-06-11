<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { beforeNavigate, goto } from "$app/navigation";
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

  beforeNavigate((nav) => {
    // WARN: this might cause in page navigation to fail
    // keep an eye on this
    const is_allowed_to_exit =
      localStorage.getItem("allowed_to_exit") === "true";
    if (nav.to?.route.id === "/(user)/dashboard" && !is_allowed_to_exit) {
      toast.error("You are not allowed to navigate back to dashboard.");
      nav.cancel();
    }
  });
</script>

<UserLayout>
  <div class="flex flex-col h-[calc(100vh-144px)] justify-center items-center">
    {#if loaded}
      {#if $page.data.session}
        <h1 class="text-4xl">Feedback Page</h1>
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
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
