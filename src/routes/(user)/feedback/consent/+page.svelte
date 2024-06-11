<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { beforeNavigate, goto } from "$app/navigation";
  import toast from "svelte-french-toast";
  let loaded: boolean = false;
  let checked: boolean = false;
  let allow_face_recording: boolean = false;

  const toggle_checkbox = (event: Event) => {
    checked = (event.target as HTMLInputElement).checked;
  };

  const toggle_optional_checkbox = (event: Event) => {
    allow_face_recording = (event.target as HTMLInputElement).checked;
  };

  const on_continue = () => {
    // NOTE: not sure if this is a good idea xD
    localStorage.setItem("consent_given", checked.toString());
    localStorage.setItem(
      "allowed_face_recording",
      allow_face_recording.toString(),
    );
    goto("/feedback/form");
  };

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
  <div class="flex flex-col h-[calc(100vh-144px)]">
    {#if loaded}
      {#if $page.data.session}
        <!-- TODO: add feedback page content  -->

        <div class="px-14 py-4">
          <p class="text-2xl font-bold">Data Privacy Consent</p>
          <p class="justify-center">
            I understand and agree that by filling out this form, I am allowing
            the Office of the Vice President for Culture, Arts, Sports, and
            Student Services of Bukidnon State University, to collect, process,
            use, share, and disclose my personal information as asked in this
            survey, and also to store it as long as necessary for the
            fulfillment of the stated purpose and in accordance with applicable
            laws, including the Data Privacy Act of 2012 and its Implementing
            Rules and Regulations.
          </p>
          <div class="flex flex-col w-fit mt-4">
            <label class="label cursor-pointer justify-normal">
              <input
                type="checkbox"
                on:click={toggle_checkbox}
                {checked}
                class="checkbox checkbox-primary"
              />
              <span class="label-text font-bold ml-2">I agree</span>
            </label>
            <label class="label cursor-pointer">
              <input
                type="checkbox"
                on:click={toggle_optional_checkbox}
                checked={allow_face_recording}
                disabled={!checked}
                class="checkbox checkbox-primary"
              />
              <span class="label-text font-bold ml-2"
                >I allow recording of my face for facial analysis for feedback
                improvement (Optional)</span
              >
            </label>
          </div>
          <button
            disabled={!checked}
            class="btn btn-primary float-end"
            on:click={on_continue}
          >
            Continue
          </button>
        </div>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
