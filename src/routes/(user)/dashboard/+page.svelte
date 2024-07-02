<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import DashboardQuickMenu from "$components/DashboardQuickMenu.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import toast from "svelte-french-toast";
  import Stats from "$components/Stats.svelte";
  import ReportGenerator from "$components/ReportGenerator.svelte";
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
  <div class="flex flex-col h-[calc(100vh-144px)]">
    {#if loaded}
      {#if $page.data.session}
        <div class="px-14 py-4">
          <div class="flex flex-row gap-6">
            <DashboardQuickMenu />
            <Stats />
          </div>
          <ReportGenerator />
        </div>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
