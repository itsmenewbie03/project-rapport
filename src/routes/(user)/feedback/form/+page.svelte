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
  import SmileyRating from "$components/SmileyRating.svelte";
  import SmileyRatingBar from "$components/SmileyRatingBar.svelte";

  let loaded: boolean = false;
  let uuid: string = "";
  let dont_stop: boolean = false;
  let recording: boolean = false;

  let client_type: string = "";
  let other_client_type: string = "";
  let contact_number: string = "";
  let name: string = "";
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
      contact_number,
      name,
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
          <!-- {#if !client_type || !purpose_of_visit || !initial_step_done} -->
          <!-- TODO: remove this switch  -->
          {#if false}
            <p class="text-xl font-bold">Name</p>
            <input
              type="text"
              placeholder="Optional"
              bind:value={name}
              class="input input-bordered w-full"
            />
            <p class="text-xl font-bold mt-4">Contact Number</p>
            <input
              type="text"
              placeholder="Optional"
              bind:value={contact_number}
              class="input input-bordered w-full"
            />
            <p class="text-xl font-bold mt-4">I am a</p>
            <div class="flex flex-inline items-center justify-between">
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="client_type"
                  bind:group={client_type}
                  value="student"
                  class="radio"
                />
                <span class="label-text ml-2">Student</span>
              </label>
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="client_type"
                  bind:group={client_type}
                  value="parent"
                  class="radio"
                />
                <span class="label-text ml-2">Parent</span>
              </label>
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="client_type"
                  bind:group={client_type}
                  value="teacher"
                  class="radio"
                />
                <span class="label-text ml-2">Teacher</span>
              </label>
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="client_type"
                  bind:group={client_type}
                  value="visitor"
                  class="radio"
                />
                <span class="label-text ml-2">Visitor</span>
              </label>
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="client_type"
                  bind:group={client_type}
                  value="government_employee"
                  class="radio"
                />
                <span class="label-text ml-2">Government Employee</span>
              </label>
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="client_type"
                  bind:group={client_type}
                  value="other"
                  class="radio"
                />
                <span class="label-text ml-2">Other</span>
              </label>
            </div>
            {#if client_type == "other"}
              <p class="text-xl font-bold mt-4">Please Specify</p>
              <input
                type="text"
                placeholder="Other"
                bind:value={other_client_type}
                class="input input-bordered w-full"
              />
            {/if}
            <p class="text-xl font-bold mt-4">Purpose of Visit</p>
            <input
              type="text"
              placeholder="Enter your purpose of visit here..."
              bind:value={purpose_of_visit}
              class="input input-bordered w-full mt-1"
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
                  description="We greatly value your opinion. Please assess your experience with our product or service by selecting the icons that corresponds to your rating using the scale given below:"
                />
              </div>
              <div class="flex flex-col gap-y-1">
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">Responsiveness</p>
                    <QuickTip
                      title="Responsiveness"
                      description="The willingness to help, assist and provide prompt service to citizens/clients."
                    />
                  </div>
                  <SmileyRatingBar bind:group={responsiveness} />
                </div>
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">Reliability (Quality)</p>
                    <QuickTip
                      title="Reliability (Quality)"
                      description="The provision of what is needed and what was promised, following the policy and standards, with zero to a minimal error rate"
                    />
                  </div>
                  <SmileyRatingBar bind:group={reliability} />
                </div>
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">Access and Facilities</p>
                    <QuickTip
                      title="Access and Facilities"
                      description="The convenience of location, ample amenities for comfortable transactions, use of clear signages and modes of technology."
                    />
                  </div>
                  <SmileyRatingBar bind:group={access_and_facilities} />
                </div>
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">Communication</p>
                    <QuickTip
                      title="Communication"
                      description="The act of keeping citizens and clients informed in a language they can easily understand, as well as listening to their feedback."
                    />
                  </div>
                  <SmileyRatingBar bind:group={communication} />
                </div>
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">
                      Value for money (If applicable)
                    </p>
                    <QuickTip
                      title="Value for money (If applicable)"
                      description="The satisfaction with timeliness of the billing, billing process/es, preferred methods of payment, reasonable payment period, the acceptable range of costs, and qualitative information on the cost of each service"
                    />
                  </div>
                  <SmileyRatingBar bind:group={value_for_money} />
                </div>
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">Integrity</p>
                    <QuickTip
                      title="Integrity"
                      description="The assurance that there is honesty, justice, fairness, and trust in each service while dealing with the citizens/clients."
                    />
                  </div>
                  <SmileyRatingBar bind:group={integrity} />
                </div>
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">Assurance</p>
                    <QuickTip
                      title="Assurance"
                      description="The capability of frontline staff to perform their duties, product and service knowledge, understanding citizen/client needs, helpfulness, and good work relationships."
                    />
                  </div>
                  <SmileyRatingBar bind:group={assurance} />
                </div>
                <div class="flex flex-col">
                  <div class="flex flex-inline items-center my-4">
                    <p class="text-xl font-bold">Outcome</p>
                    <QuickTip
                      title="Outcome"
                      description="The extent of achieving outcomes or realizing the intended benefits of government services."
                    />
                  </div>
                  <SmileyRatingBar bind:group={outcome} />
                </div>
                <div class="flex flex-col">
                  <p class="text-xl font-bold my-4">
                    Overall Satisfaction Rating
                  </p>
                  <SmileyRatingBar bind:group={overall_satisfaction} />
                </div>
              </div>
            </div>
            <button
              class="btn btn-primary float-end mt-4 mb-16"
              on:click={submit}
            >
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
