<script lang="ts">
  import toast from "svelte-french-toast";
  import { logout } from "$lib/utils";
  import { goto } from "$app/navigation";
  const _logout = async (event: Event) => {
    await toast.promise(logout(), {
      loading: "Signing you out...",
      success: "You've been logged out successfully.",
      error: "We encountered an issue logging you out. Please try again.",
    });
    await goto("/login");
  };
</script>

<div class="navbar bg-base-100">
  <div class="navbar-start">
    <div class="dropdown">
      <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          ><path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 6h16M4 12h16M4 18h7"
          /></svg
        >
      </div>
      <ul
        tabindex="-1"
        class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
      >
        <li><a href="/dashboard">Dashboard</a></li>
        <li><a href="/analytics">Analytics</a></li>
        <li><a href="/configurations">Configurations</a></li>
      </ul>
    </div>
  </div>
  <div class="navbar-center">
    <p class="font-bold text-xl">Project Rapport</p>
  </div>
  <div class="navbar-end">
    <div class="dropdown dropdown-end">
      <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
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
      </div>
      <ul
        tabindex="-1"
        class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
      >
        <li><a href="/profile">Profile</a></li>
        <li><button on:click={_logout}>Logout</button></li>
      </ul>
    </div>
  </div>
</div>
