<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import QuickTip from "$components/QuickTip.svelte";
  import toast from "svelte-french-toast";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { beforeNavigate, goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { gen_uuid } from "$lib/uuids";
  let loaded: boolean = false;
  let responsiveness: number = 3;
  let reliability: number = 3;
  let access_and_facilities: number = 3;
  let communication: number = 3;
  let value_for_money: number = 3;
  let integrity: number = 3;
  let assurance: number = 3;
  let outcome: number = 3;
  let overall_satisfaction: number = 3;
  let uuid: string = "";

  const submit = async (event: Event) => {
    event.preventDefault();
    const data = {
      responsiveness,
      reliability,
      access_and_facilities,
      communication,
      value_for_money,
      integrity,
      assurance,
      outcome,
      overall_satisfaction,
    };
    console.log(data);
    try {
      const response: any = await invoke("submit_feedback", {
        id: uuid,
        feedback: JSON.stringify(data),
      });
      toast.success(response);
    } catch (err: any) {
      toast.error(err);
    }
  };

  onMount(async () => {
    // INFO: generate a unique id for this feedback session
    uuid = gen_uuid();

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
    // INFO: when user allowed_face_recording we should tell our backend to start recording
    if (localStorage.getItem("allowed_face_recording") === "true") {
      try {
        await invoke("start_face_recording", {
          id: uuid,
        });
        toast.success("You are now being recorded.");
      } catch (error: any) {
        toast.error("There was an error starting the recording.");
        // TODO: implement logging for debugging
      }
    }
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
            <div class="flex flex-inline items-center">
              <p class="text-2xl font-bold">Tell us what you think!</p>
              <QuickTip
                title="Rating"
                description="This feedback scale uses colors! Red means very unhappy, orange unhappy, yellow okay, lime happy, and green very happy."
              />
            </div>

            <p>
              We greatly value your opinion. Please assess your experience with
              our product or service by selecting the icons that corresponds to
              your rating using the scale given below:
            </p>
            <div class="flex flex-col gap-y-1">
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">Responsiveness</p>
                  <QuickTip
                    title="Responsiveness"
                    description="The willingness to help, assist and provide prompt service to citizens/clients."
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-1"
                    value={1}
                    bind:group={responsiveness}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    value={2}
                    bind:group={responsiveness}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    value={3}
                    bind:group={responsiveness}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    value={4}
                    bind:group={responsiveness}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-1"
                    value={5}
                    bind:group={responsiveness}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">Reliability (Quality)</p>
                  <QuickTip
                    title="Reliability (Quality)"
                    description="The provision of what is needed and what was promised, following the policy and standards, with zero to a minimal error rate"
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-2"
                    value={1}
                    bind:group={reliability}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    value={2}
                    bind:group={reliability}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    value={3}
                    bind:group={reliability}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    value={4}
                    bind:group={reliability}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-2"
                    value={5}
                    bind:group={reliability}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">Access and Facilities</p>
                  <QuickTip
                    title="Access and Facilities"
                    description="The convenience of location, ample amenities for comfortable transactions, use of clear signages and modes of technology."
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-3"
                    value={1}
                    bind:group={access_and_facilities}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    value={2}
                    bind:group={access_and_facilities}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    value={3}
                    bind:group={access_and_facilities}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    value={4}
                    bind:group={access_and_facilities}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-3"
                    value={5}
                    bind:group={access_and_facilities}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">Communication</p>
                  <QuickTip
                    title="Communication"
                    description="The act of keeping citizens and clients informed in a language they can easily understand, as well as listening to their feedback."
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-4"
                    value={1}
                    bind:group={communication}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    value={2}
                    bind:group={communication}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    value={3}
                    bind:group={communication}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    value={4}
                    bind:group={communication}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-4"
                    value={5}
                    bind:group={communication}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">
                    Value for money (If applicable)
                  </p>
                  <QuickTip
                    title="Value for money (If applicable)"
                    description="The satisfaction with timeliness of the billing, billing process/es, preferred methods of payment, reasonable payment period, the acceptable range of costs, and qualitative information on the cost of each service"
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-5"
                    value={1}
                    bind:group={value_for_money}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    value={2}
                    bind:group={value_for_money}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    value={3}
                    bind:group={value_for_money}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    value={4}
                    bind:group={value_for_money}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-5"
                    value={5}
                    bind:group={value_for_money}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">Integrity</p>
                  <QuickTip
                    title="Integrity"
                    description="The assurance that there is honesty, justice, fairness, and trust in each service while dealing with the citizens/clients."
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-6"
                    value={1}
                    bind:group={integrity}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    value={2}
                    bind:group={integrity}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    value={3}
                    bind:group={integrity}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    value={4}
                    bind:group={integrity}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-6"
                    value={5}
                    bind:group={integrity}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">Assurance</p>
                  <QuickTip
                    title="Assurance"
                    description="The capability of frontline staff to perform their duties, product and service knowledge, understanding citizen/client needs, helpfulness, and good work relationships."
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-7"
                    value={1}
                    bind:group={assurance}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    value={2}
                    bind:group={assurance}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    value={3}
                    bind:group={assurance}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    value={4}
                    bind:group={assurance}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-7"
                    value={5}
                    bind:group={assurance}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
              <div
                class="rating rating-lg gap-8 grid grid-cols-2 flex-inline items-center"
              >
                <div class="flex flex-inline items-center">
                  <p class="text-xl font-bold">Outcome</p>
                  <QuickTip
                    title="Outcome"
                    description="The extent of achieving outcomes or realizing the intended benefits of government services."
                  />
                </div>
                <div class="flex justify-between">
                  <input
                    type="radio"
                    name="rating-8"
                    value={1}
                    bind:group={outcome}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    value={2}
                    bind:group={outcome}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    value={3}
                    bind:group={outcome}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    value={4}
                    bind:group={outcome}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-8"
                    value={5}
                    bind:group={outcome}
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
                    value={1}
                    bind:group={overall_satisfaction}
                    class="mask mask-heart bg-red-400"
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    value={2}
                    bind:group={overall_satisfaction}
                    class="mask mask-heart bg-orange-400"
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    value={3}
                    bind:group={overall_satisfaction}
                    class="mask mask-heart bg-yellow-400"
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    value={4}
                    bind:group={overall_satisfaction}
                    class="mask mask-heart bg-lime-400"
                  />
                  <input
                    type="radio"
                    name="rating-9"
                    value={5}
                    bind:group={overall_satisfaction}
                    class="mask mask-heart bg-green-400"
                  />
                </div>
              </div>
            </div>
          </div>
          <button class="btn btn-primary float-end mt-4" on:click={submit}>
            Submit
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
