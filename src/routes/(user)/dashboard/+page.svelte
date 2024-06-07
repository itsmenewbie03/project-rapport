<script lang="ts">
  import { signOut } from "@auth/sveltekit/client";
  import { page } from "$app/stores";
  import UserLayout from "$components/UserLayout.svelte";
</script>

<UserLayout>
  {#if $page.data.session}
    <h1 class="text-4xl">Protected page</h1>
    <p class="text-md">
      This is a protected content. You can access this content because you are
      signed in.
    </p>
    <p class="text-sm">Session expiry: {$page.data.session?.expires}</p>

    <button
      on:click={(_e) => {
        signOut();
      }}
      class="btn btn-primary"
    >
      LOGOUT!
    </button>
  {:else}
    <h1>Access Denied</h1>
  {/if}
</UserLayout>
