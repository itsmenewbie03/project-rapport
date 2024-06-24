<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import toast from "svelte-french-toast";
  import { invoke } from "@tauri-apps/api/tauri";
  import { validate_email as is_valid_email } from "$lib/email_validator";
  let loaded: boolean = false;

  let max_negative_feedback: number = 3;
  let email_recipient: string = "email@example.com";

  type ConfigData = {
    name: string;
    value: string;
  };

  const save = async (event: Event) => {
    if (!is_valid_email(email_recipient)) {
      toast.error("Invalid email address.");
      return;
    }
    const config_data = {
      // NOTE: we won't do dyn in rust to JS will adjust xD
      max_negative_feedback: max_negative_feedback.toString(),
      email_recipient,
    };
    try {
      const config_saved: string = await invoke("save_configs", {
        data: config_data,
      });
      toast.success(config_saved);
    } catch (err: any) {
      toast.error(err);
    }
  };
  onMount(async () => {
    if (!$page.data.session) {
      toast.error("Please login first.");
      await goto("/login");
    }

    try {
      const configs: ConfigData[] = await invoke("get_configs");
      configs.forEach((config) => {
        if (config.name === "max_negative_feedback") {
          max_negative_feedback = parseInt(config.value);
        } else if (config.name === "email_recipient") {
          email_recipient = config.value;
        }
      });
      console.log("CONFIGS", configs);
    } catch (err: any) {
      console.error("CONFIGS", err);
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
          <label class="form-control w-full">
            <div class="label">
              <span class="label-text"
                >Consecutive Negative Feedback Threshold</span
              >
            </div>
            <input
              type="number"
              class="input input-bordered w-full"
              bind:value={max_negative_feedback}
              min="2"
            />
            <div class="label">
              <span class="label-text-alt"
                >Triggers email alert for {max_negative_feedback}
                consecutive negative feedbacks.</span
              >
            </div>
          </label>
          <label class="form-control w-full">
            <div class="label">
              <span class="label-text">Email Notification Address</span>
            </div>
            <input
              type="email"
              class="input input-bordered w-full"
              bind:value={email_recipient}
            />
            <div class="label">
              <span class="label-text-alt"
                >Alerts will be sent to this email.</span
              >
            </div>
          </label>
          <button class="btn btn-primary mt-2 float-right w-20" on:click={save}
            >Save</button
          >
        </div>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
