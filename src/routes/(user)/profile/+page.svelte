<script lang="ts">
  import UserLayout from '$components/UserLayout.svelte';
  import LoadingBars from '$components/LoadingBars.svelte';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import toast from 'svelte-french-toast';
  import { invoke } from '@tauri-apps/api/tauri';
  import { validate_email as is_valid_email } from '$lib/email_validator';
  let loaded: boolean = false;
  let name: string = '';
  let email: string = '';

  let current_password: string = '';
  let new_password: string = '';
  let confirm_new_password: string = '';

  const save_profile = async (event: Event) => {
    if (!is_valid_email(email)) {
      toast.error('Invalid email address.');
      return;
    }
    if (name === '' || email === '') {
      toast.error('Name and email cannot be empty.');
      return;
    }
    if (name.length < 3) {
      toast.error('Name must be at least 3 characters.');
      return;
    }
    const update_data = {
      email,
      name,
    };
    try {
      const resp: string = await invoke('update_profile', {
        email: $page.data.session.email,
        data: update_data,
      });
      toast.success(resp);
      // TODO: require re-login after profile update
      localStorage.removeItem('auth_token');
      setTimeout(() => {
        goto('/login');
      }, 2000);
    } catch (err: any) {
      toast.error(err);
    }
  };

  const update_password = async (event: Event) => {
    const is_verified = await invoke('authenticate', {
      email: $page.data.session.email,
      password: current_password,
    });

    if (!is_verified) {
      toast.error('Current password is incorrect.');
      return;
    }

    if (new_password !== confirm_new_password) {
      toast.error('New password and confirm password must match.');
      return;
    }
    try {
      const resp: string = await invoke('change_password', {
        email: $page.data.session.email,
        current: current_password,
        new: new_password,
      });
      toast.success(resp);
      // TODO: require re-login after profile update
      localStorage.removeItem('auth_token');
      setTimeout(() => {
        goto('/login');
      }, 2000);
    } catch (err: any) {
      toast.error(err);
    }
  };

  onMount(async () => {
    if (!$page.data.session) {
      toast.error('Please login first.');
      await goto('/login');
    }
    name = $page.data.session.name;
    email = $page.data.session.email;
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
          <div class="card bg-base-100 w-full shadow-xl bordered">
            <div class="card-body">
              <h2 class="card-title">Profile Information</h2>
              <p>
                Update your account's profile information and email address.
              </p>
              <label class="form-control w-full">
                <div class="label">
                  <span class="label-text">Name</span>
                </div>
                <input
                  type="text"
                  class="input input-bordered w-full"
                  bind:value={name}
                />
              </label>
              <label class="form-control w-full">
                <div class="label">
                  <span class="label-text">Email</span>
                </div>
                <input
                  type="email"
                  class="input input-bordered w-full"
                  bind:value={email}
                />
              </label>
              <div class="card-actions justify-end mt-2">
                <button class="btn btn-primary" on:click={save_profile}
                  >Save</button
                >
              </div>
            </div>
          </div>
          <div class="card bg-base-100 w-full shadow-xl bordered mt-4 mb-14">
            <div class="card-body">
              <h2 class="card-title">Update Password</h2>
              <p>
                Ensure your account is using a long, random password to stay
                secure.
              </p>
              <label class="form-control w-full">
                <div class="label">
                  <span class="label-text">Current Password</span>
                </div>
                <input
                  type="password"
                  class="input input-bordered w-full"
                  bind:value={current_password}
                />
              </label>
              <label class="form-control w-full">
                <div class="label">
                  <span class="label-text">New Password</span>
                </div>
                <input
                  type="password"
                  class="input input-bordered w-full"
                  bind:value={new_password}
                />
              </label>
              <label class="form-control w-full">
                <div class="label">
                  <span class="label-text">Confirm Password</span>
                </div>
                <input
                  type="password"
                  class="input input-bordered w-full"
                  bind:value={confirm_new_password}
                />
              </label>
              <div class="card-actions justify-end mt-2">
                <button class="btn btn-primary" on:click={update_password}
                  >Save</button
                >
              </div>
            </div>
          </div>
        </div>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
