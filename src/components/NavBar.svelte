<script lang="ts">
  import toast from "svelte-french-toast";
  import { logout } from "$lib/utils";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import ConfirmModal from "./ConfirmModal.svelte";

  const is_feedback_page = $page.url.pathname.includes("feedback");
  let secret_hits: number = 0;
  let timeout: Timer;
  let modal_handle: any;

  const secret_exit = (event: Event) => {
    // INFO: this is only available in feedback page
    if (!is_feedback_page) return;
    timeout = setTimeout(() => {
      secret_hits = 0;
    }, 5000);
    secret_hits++;
    if (secret_hits === 5) {
      modal_handle.show();
      secret_hits = 0;
      clearTimeout(timeout);
    }
  };

  const warn = (event: Event) => {
    toast.error("This feature is not available during feedback process.");
  };

  const _logout = async (event: Event) => {
    await toast.promise(logout(), {
      loading: "Signing you out...",
      success: "You've been logged out successfully.",
      error: "We encountered an issue logging you out. Please try again.",
    });
    await goto("/login");
  };
</script>

{#if is_feedback_page}
  <ConfirmModal bind:this={modal_handle} />
{/if}
<div class="navbar bg-base-100">
  <div class="navbar-start">
    {#if !is_feedback_page}
      <div class="dropdown">
        <button class="btn btn-circle btn-ghost">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            class="inline-block w-5 h-5 stroke-current"
            ><path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 6h16M4 12h16M4 18h16"
            ></path></svg
          >
        </button>
        <ul
          tabindex="-1"
          class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
        >
          <li><a href="/dashboard">Dashboard</a></li>
          <li><a href="/analytics">Analytics</a></li>
          <li><a href="/configurations">Configurations</a></li>
        </ul>
      </div>
    {:else}
      <button class="btn btn-circle btn-ghost" on:click={warn}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          class="inline-block w-5 h-5 stroke-current"
          ><path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 6h16M4 12h16M4 18h16"
          ></path></svg
        >
      </button>
    {/if}
  </div>
  <div class="navbar-center">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions  -->
    <!-- svelte-ignore a11y-click-events-have-key-events  -->
    <p class="font-bold text-xl cursor-default" on:click={secret_exit}>
      Project Rapport
    </p>
  </div>
  <div class="navbar-end">
    {#if !is_feedback_page}
      <div class="dropdown dropdown-end">
        <button class="btn btn-circle btn-ghost">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="size-6"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
            />
          </svg>
        </button>
        <ul
          tabindex="-1"
          class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
        >
          <li><a href="/profile">Profile</a></li>
          <li><button on:click={_logout}>Logout</button></li>
        </ul>
      </div>
    {:else}
      <button class="btn btn-circle btn-ghost" on:click={warn}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
          />
        </svg>
      </button>
    {/if}
  </div>
</div>
