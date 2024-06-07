<script lang="ts">
  import GuestLayout from "$components/GuestLayout.svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  let loaded: boolean = false;
  onMount(async () => {
    loaded = true;
    if ($page.data.session) {
      await goto("/dashboard");
    }
  });
</script>

<GuestLayout>
  <div class="flex flex-col min-h-screen justify-center items-center">
    {#if loaded}
      {#if !$page.data.session}
        <p>
          For some reason you are not redirected to the login route. Please
          reload the page. You should not be able to see this as if you don't
          have a session you will be redirected to the login page.
        </p>
      {/if}
    {:else}
      <span class="loading loading-bars loading-lg"></span>
    {/if}
  </div>
</GuestLayout>
