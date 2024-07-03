<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import QuickTip from "$components/QuickTip.svelte";
  import ThankYouModal from "$components/ThankYouModal.svelte";
  import toast from "svelte-french-toast";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { beforeNavigate, goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { confirm } from "@tauri-apps/api/dialog";
  import { gen_uuid } from "$lib/uuids";

  let loaded: boolean = false;
  let uuid: string = "";
  let dont_stop: boolean = false;
  let recording: boolean = false;

  let client_type: string = "";
  let other_client_type: string = "";
  let purpose_of_visit: string = "";
  let initial_step_done = false;

  // INFO: Thank You Modal Stuff
  let modal_handle: any;
  let message: string = "";

  // INFO: feedback data
  let responsiveness: number = 0;
  $: responsiveness,
    (async () => {
      if (responsiveness) {
        await take_photo("responsiveness");
      }
    })();

  let reliability: number = 0;
  $: reliability,
    (async () => {
      if (reliability) {
        await take_photo("reliability");
      }
    })();

  let access_and_facilities: number = 0;
  $: access_and_facilities,
    (async () => {
      if (access_and_facilities) {
        await take_photo("access_and_facilities");
      }
    })();

  let communication: number = 0;
  $: communication,
    (async () => {
      if (communication) {
        await take_photo("communication");
      }
    })();

  let value_for_money: number = 0;
  $: value_for_money,
    (async () => {
      if (value_for_money) {
        await take_photo("value_for_money");
      }
    })();

  let integrity: number = 0;
  $: integrity,
    (async () => {
      if (integrity) {
        await take_photo("integrity");
      }
    })();

  let assurance: number = 0;
  $: assurance,
    (async () => {
      if (assurance) {
        await take_photo("assurance");
      }
    })();

  let outcome: number = 0;
  $: outcome,
    (async () => {
      if (outcome) {
        await take_photo("outcome");
      }
    })();

  let overall_satisfaction: number = 0;
  $: overall_satisfaction,
    (async () => {
      if (overall_satisfaction) {
        await take_photo("overall_satisfaction");
      }
    })();
  // NOTE:
  // key: quality, value: dominant_emotion
  const emotion_data: Record<string, string> = {};
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

    // WARN: shitty code ahead
    let office_name: string = "NOT CONFIGURED";
    type ConfigData = {
      name: string;
      value: string;
    };
    try {
      const configs: ConfigData[] = await invoke("get_configs");
      configs.forEach((config) => {
        if (config.name === "office_name") {
          office_name = config.value;
        }
      });
    } catch (err) {
      console.log("failed to get office name from configs database!");
    }
    // WARN: end of shitty code

    const metadata = {
      client_type: client_type === "other" ? other_client_type : client_type,
      purpose_of_visit,
      office_name,
    };
    // TODO: check for 0s in data
    const non_rated = [];
    for (let key in data) {
      // @ts-ignore
      if (data[key] === 0) {
        non_rated.push(key);
      }
    }
    if (non_rated.length > 0) {
      toast.error(`Please rate the following: ${non_rated.join(", ")}.`);
      return;
    }
    // TODO: ensure all emotions are captured
    if (Object.keys(emotion_data).length !== 9 && recording) {
      toast.error(
        "Oops, you are going too fast! Please click submit after a few seconds.",
      );
      return;
    }
    try {
      console.log(emotion_data);
      console.log(data);
      const response: any = await invoke("submit_feedback", {
        id: uuid,
        feedback: JSON.stringify(data),
        emotion:
          // TODO: you see this crazy ternary? this is to make them confused xD
          Object.keys(emotion_data).length && recording
            ? JSON.stringify(emotion_data)
            : null,
        metadata: JSON.stringify(metadata),
        recording: recording,
      });
      message = `Your feedback has been submitted successfully. ${recording ? "Recording is also stopped." : ""} You will be redirected back to the consent screen shortly.`;
      modal_handle.show();
      setTimeout(async () => {
        modal_handle.hide();
        dont_stop = true;
        await goto("/feedback/consent");
      }, 4000);
    } catch (err: any) {
      toast.error(err);
    }
  };

  const initial_submit = async (event: Event) => {
    if (
      !client_type ||
      !purpose_of_visit ||
      (client_type === "other" && !other_client_type)
    ) {
      toast.error("Please fill out all the fields.");
      return;
    }
    // INFO: when user allowed_face_recording we should tell our backend to start recording
    if (localStorage.getItem("allowed_face_recording") === "true") {
      try {
        await invoke("start_face_recording", {
          id: uuid,
        });
        toast.success("You are now being recorded.");
        recording = true;
      } catch (error: any) {
        toast.error("There was an error starting the recording.");
        // TODO: implement logging for debugging
      }
    }
    // NOTE: consent is only valid for one feedback session
    localStorage.removeItem("consent_given");
    localStorage.removeItem("allowed_face_recording");
    initial_step_done = true;
  };

  const take_photo = async (quality: string) => {
    if (!recording) return;
    try {
      const frame_data: string = await invoke("take_photo", {
        id: uuid,
        quality: quality,
      });
      // NOTE: the frame_data we recieve is a JSON string
      // we need to parse it in the client side
      // as we will need an extra struct just for parsing
      // to do it in the server
      const frame = JSON.parse(frame_data);
      // INFO: store the dominant emotion
      emotion_data[quality] = frame.dominant_emotion;
    } catch (err) {
      // BUG: for debug we will just toast error the bug
      toast.error("There was an error taking the photo.");
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
      toast.error("Session expired, please start from the beginning.");
      // HACK: stop the recording just in case the user reloads the feedback page.
      // why do we have to suffer for their mistakes? xD
      await invoke("clear_recording", {
        id: "I DON'T HAVE ENOUGH TIME TO IMPLEMENT THIS PROPERLY xD!",
      });
      await goto("/feedback/consent");
      return;
    }
    // TEST: for aesthetics we will delay the load for a second xD
    setTimeout(() => {
      loaded = true;
    }, 1000);
  });

  beforeNavigate(async (nav) => {
    // WARN: this might cause in page navigation to fail
    // keep an eye on this
    const is_allowed_to_exit =
      localStorage.getItem("allowed_to_exit") === "true";
    if (nav.to?.route.id === "/(user)/dashboard" && !is_allowed_to_exit) {
      toast.error("You are not allowed to navigate back to dashboard.");
      nav.cancel();
      return;
    }
    if (
      recording &&
      nav.to?.route.id === "/(user)/feedback/consent" &&
      !dont_stop
    ) {
      // NOTE: for some reason the navigation continues itself so we stop it and manually navigate
      nav.cancel();
      let toast_id = toast.loading("Confirmation needed, waiting...");
      let confirmed = await confirm(
        "Returning to the consent screen will stop the recording and delete the file.",
        "Are you sure you want to proceed?",
      );
      console.log(`Confirmed: ${confirmed}`);
      if (!confirmed) {
        toast.remove(toast_id);
        return;
      }
      toast.remove(toast_id);
      toast.promise(invoke("clear_recording", { id: uuid }), {
        loading: "Stopping recording and deleting file...",
        success: "Recording stopped successfully. The file has been deleted.",
        error:
          "We encountered a problem stopping the recording and deleting the file",
      });
      // INFO: manual redirect
      dont_stop = true;
      await goto("/feedback/consent");
    }
  });
</script>

<UserLayout>
  <ThankYouModal bind:this={modal_handle} {message} />
  <div class="flex flex-col h-[calc(100vh-144px)]">
    {#if loaded}
      {#if $page.data.session}
        <div class="px-14 py-4">
          {#if !client_type || !purpose_of_visit || !initial_step_done}
            <p class="text-xl font-bold">I am a</p>
            <select
              class="select select-primary w-full max-w-xs mt-1"
              bind:value={client_type}
            >
              <option disabled selected value="">Select an option</option>
              <option value="student">Student</option>
              <option value="parent">Parent</option>
              <option value="teacher">Teacher</option>
              <option value="visitor">Visitor</option>
              <option value="government_employee">Government Employee</option>
              <option value="other">Other</option>
            </select>
            {#if client_type == "other"}
              <input
                type="text"
                placeholder="Please Specify"
                bind:value={other_client_type}
                class="input input-bordered w-full max-w-xs ml-4"
              />
            {/if}
            <p class="text-xl font-bold mt-4">Purpose of Visit</p>
            <input
              type="text"
              placeholder="Enter your purpose of visit here..."
              bind:value={purpose_of_visit}
              class="input input-bordered w-full max-w-xs mt-1"
            />
            <div class="mt-4">
              <button
                class="btn btn-primary"
                on:click={initial_submit}
                disabled={!client_type ||
                  !purpose_of_visit ||
                  (client_type === "other" && !other_client_type)}>Next</button
              >
            </div>
          {:else}
            <div>
              <div class="flex flex-inline items-center">
                <p class="text-2xl font-bold">Tell us what you think!</p>
                <QuickTip
                  title="Rating"
                  description="This feedback scale uses colors! Red means very unhappy, orange unhappy, yellow okay, lime happy, and green very happy."
                />
              </div>

              <p class="text-md">
                This feedback scale uses colors! <b>
                  From left to right, the hearts represent a scale of 1 to 5.
                  Red means very unhappy, orange unhappy, yellow okay, lime
                  happy, and green very happy.</b
                >
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
                      value={0}
                      bind:group={responsiveness}
                      class={`mask mask-heart bg-red-400 ${responsiveness == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-1"
                      value={1}
                      bind:group={responsiveness}
                      class={`mask mask-heart bg-red-400 ${responsiveness == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-1"
                      value={2}
                      bind:group={responsiveness}
                      class={`mask mask-heart bg-orange-400 ${responsiveness == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-1"
                      value={3}
                      bind:group={responsiveness}
                      class={`mask mask-heart bg-yellow-400 ${responsiveness == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-1"
                      value={4}
                      bind:group={responsiveness}
                      class={`mask mask-heart bg-lime-400 ${responsiveness == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-1"
                      value={5}
                      bind:group={responsiveness}
                      class={`mask mask-heart bg-green-400 ${responsiveness == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={reliability}
                      class={`mask mask-heart bg-red-400 ${reliability == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-2"
                      value={1}
                      bind:group={reliability}
                      class={`mask mask-heart bg-red-400 ${reliability == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-2"
                      value={2}
                      bind:group={reliability}
                      class={`mask mask-heart bg-orange-400 ${reliability == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-2"
                      value={3}
                      bind:group={reliability}
                      class={`mask mask-heart bg-yellow-400 ${reliability == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-2"
                      value={4}
                      bind:group={reliability}
                      class={`mask mask-heart bg-lime-400 ${reliability == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-2"
                      value={5}
                      bind:group={reliability}
                      class={`mask mask-heart bg-green-400 ${reliability == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={access_and_facilities}
                      class={`mask mask-heart bg-red-400 ${access_and_facilities == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-3"
                      value={1}
                      bind:group={access_and_facilities}
                      class={`mask mask-heart bg-red-400 ${access_and_facilities == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-3"
                      value={2}
                      bind:group={access_and_facilities}
                      class={`mask mask-heart bg-orange-400 ${access_and_facilities == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-3"
                      value={3}
                      bind:group={access_and_facilities}
                      class={`mask mask-heart bg-yellow-400 ${access_and_facilities == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-3"
                      value={4}
                      bind:group={access_and_facilities}
                      class={`mask mask-heart bg-lime-400 ${access_and_facilities == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-3"
                      value={5}
                      bind:group={access_and_facilities}
                      class={`mask mask-heart bg-green-400 ${access_and_facilities == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={communication}
                      class={`mask mask-heart bg-red-400 ${communication == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-4"
                      value={1}
                      bind:group={communication}
                      class={`mask mask-heart bg-red-400 ${communication == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-4"
                      value={2}
                      bind:group={communication}
                      class={`mask mask-heart bg-orange-400 ${communication == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-4"
                      value={3}
                      bind:group={communication}
                      class={`mask mask-heart bg-yellow-400 ${communication == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-4"
                      value={4}
                      bind:group={communication}
                      class={`mask mask-heart bg-lime-400 ${communication == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-4"
                      value={5}
                      bind:group={communication}
                      class={`mask mask-heart bg-green-400 ${communication == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={value_for_money}
                      class={`mask mask-heart bg-red-400 ${value_for_money == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-5"
                      value={1}
                      bind:group={value_for_money}
                      class={`mask mask-heart bg-red-400 ${value_for_money == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-5"
                      value={2}
                      bind:group={value_for_money}
                      class={`mask mask-heart bg-orange-400 ${value_for_money == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-5"
                      value={3}
                      bind:group={value_for_money}
                      class={`mask mask-heart bg-yellow-400 ${value_for_money == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-5"
                      value={4}
                      bind:group={value_for_money}
                      class={`mask mask-heart bg-lime-400 ${value_for_money == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-5"
                      value={5}
                      bind:group={value_for_money}
                      class={`mask mask-heart bg-green-400 ${value_for_money == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={integrity}
                      class={`mask mask-heart bg-red-400 ${integrity == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-6"
                      value={1}
                      bind:group={integrity}
                      class={`mask mask-heart bg-red-400 ${integrity == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-6"
                      value={2}
                      bind:group={integrity}
                      class={`mask mask-heart bg-orange-400 ${integrity == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-6"
                      value={3}
                      bind:group={integrity}
                      class={`mask mask-heart bg-yellow-400 ${integrity == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-6"
                      value={4}
                      bind:group={integrity}
                      class={`mask mask-heart bg-lime-400 ${integrity == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-6"
                      value={5}
                      bind:group={integrity}
                      class={`mask mask-heart bg-green-400 ${integrity == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={assurance}
                      class={`mask mask-heart bg-red-400 ${assurance == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-7"
                      value={1}
                      bind:group={assurance}
                      class={`mask mask-heart bg-red-400 ${assurance == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-7"
                      value={2}
                      bind:group={assurance}
                      class={`mask mask-heart bg-orange-400 ${assurance == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-7"
                      value={3}
                      bind:group={assurance}
                      class={`mask mask-heart bg-yellow-400 ${assurance == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-7"
                      value={4}
                      bind:group={assurance}
                      class={`mask mask-heart bg-lime-400 ${assurance == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-7"
                      value={5}
                      bind:group={assurance}
                      class={`mask mask-heart bg-green-400 ${assurance == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={outcome}
                      class={`mask mask-heart bg-red-400 ${outcome == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-8"
                      value={1}
                      bind:group={outcome}
                      class={`mask mask-heart bg-red-400 ${outcome == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-8"
                      value={2}
                      bind:group={outcome}
                      class={`mask mask-heart bg-orange-400 ${outcome == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-8"
                      value={3}
                      bind:group={outcome}
                      class={`mask mask-heart bg-yellow-400 ${outcome == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-8"
                      value={4}
                      bind:group={outcome}
                      class={`mask mask-heart bg-lime-400 ${outcome == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-8"
                      value={5}
                      bind:group={outcome}
                      class={`mask mask-heart bg-green-400 ${outcome == 5 ? "" : "bg-opacity-20"}`}
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
                      value={0}
                      bind:group={overall_satisfaction}
                      class={`mask mask-heart bg-red-400 ${overall_satisfaction == 0 ? "" : "bg-opacity-20"}`}
                      hidden
                    />
                    <input
                      type="radio"
                      name="rating-9"
                      value={1}
                      bind:group={overall_satisfaction}
                      class={`mask mask-heart bg-red-400 ${overall_satisfaction == 1 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-9"
                      value={2}
                      bind:group={overall_satisfaction}
                      class={`mask mask-heart bg-orange-400 ${overall_satisfaction == 2 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-9"
                      value={3}
                      bind:group={overall_satisfaction}
                      class={`mask mask-heart bg-yellow-400 ${overall_satisfaction == 3 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-9"
                      value={4}
                      bind:group={overall_satisfaction}
                      class={`mask mask-heart bg-lime-400 ${overall_satisfaction == 4 ? "" : "bg-opacity-20"}`}
                    />
                    <input
                      type="radio"
                      name="rating-9"
                      value={5}
                      bind:group={overall_satisfaction}
                      class={`mask mask-heart bg-green-400 ${overall_satisfaction == 5 ? "" : "bg-opacity-20"}`}
                    />
                  </div>
                </div>
              </div>
            </div>
            <button class="btn btn-primary float-end mt-4" on:click={submit}>
              Submit
            </button>
          {/if}
        </div>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
