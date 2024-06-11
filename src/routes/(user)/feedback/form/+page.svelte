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
    // TODO: check for flags in localStorage
    if (localStorage.getItem("consent_given") !== "true") {
      toast.error("Please provide consent first.");
      // await goto("/feedback/consent");
      // return;
    }
    // TEST: for aesthetics we will delay the load for a second xD
    setTimeout(() => {
      loaded = true;
    }, 1000);
    // NOTE: consent is only valid for one feedback session
    localStorage.removeItem("consent_given");
    localStorage.removeItem("allowed_face_recording");
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
          <div>
            <p class="text-2xl font-bold">Tell us what you think!</p>
            <p>
              We greatly value your opinion. Please assess your experience with
              our product or service by selecting the icons that corresponds to
              your rating using the scale given below:
            </p>
            <div class="flex flex-col gap-y-1">
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Responsiveness</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-1"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Reliability (Quality)</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-2"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Access and Facilities</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-3"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Communication</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-4"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Value for money (If applicable)</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-5"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Integrity</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-6"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Assurance</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-7"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Outcome</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-8"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <p class="text-xl font-bold">Overall Satisfaction Rating</p>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-9"
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    class="mask mask-heart bg-orange-400"
                    checked
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
            </div>
          </div>
          <button class="btn btn-primary float-end mt-4"> Submit </button>
        </div>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
