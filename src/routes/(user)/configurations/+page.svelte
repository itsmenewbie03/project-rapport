<script lang="ts">
  import UserLayout from '$components/UserLayout.svelte';
  import LoadingBars from '$components/LoadingBars.svelte';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import toast from 'svelte-french-toast';
  import { invoke } from '@tauri-apps/api/tauri';
  import { validate_email as is_valid_email } from '$lib/email_validator';
  import { ask, confirm } from '@tauri-apps/api/dialog';
  let loaded: boolean = false;

  let max_negative_feedback: number = 3;
  let email_recipient: string = 'email@example.com';
  let office_name: string = 'NOT CONFIGURED YET';
  let emotion_weight: number = 0;
  let emotion_weight_input: HTMLInputElement;
  let enable_consent_screen: string = 'false';
  let services_list: string[] = [];

  type ConfigData = {
    name: string;
    value: string;
  };

  type ServiceData = {
    name: string;
  };

  const save = async (event: Event) => {
    if (!is_valid_email(email_recipient)) {
      toast.error('Invalid email address.');
      return;
    }
    if (emotion_weight < 0.01 || emotion_weight > 0.99) {
      toast.error('Emotion weight must be between 0.01 and 0.99.');
      emotion_weight_input.focus();
      return;
    }
    const config_data = {
      // NOTE: we won't do dyn in rust to JS will adjust xD
      max_negative_feedback: max_negative_feedback.toString(),
      email_recipient,
      office_name,
      enable_consent_screen,
      emotion_weight: emotion_weight.toString(),
    };
    try {
      const config_saved: string = await invoke('save_configs', {
        data: config_data,
      });
      toast.success(config_saved);
    } catch (err: any) {
      toast.error(err);
    }
  };

  const add_service = async (event: Event) => {
    let service = prompt('Please enter the service you want to add.');
    if (!service) {
      return;
    }
    let service_cleaned = service.trim();
    if (!service_cleaned.length) {
      toast.error('Service must be not be all whitespaces.');
      return;
    }
    if (service_cleaned.length < 5) {
      toast.error('Service must be at least 5 characters.');
      return;
    }
    try {
      const resp: string = await invoke('add_service', {
        service: service_cleaned,
      });
      services_list.push(service_cleaned);
      services_list = services_list;
      toast.success(resp);
    } catch (err: any) {
      console.error('ADD_SERVICE', err);
    }
  };

  const load_services = async () => {
    try {
      const services: ServiceData[] = await invoke('get_services');
      services.forEach((service) => {
        services_list.push(service.name);
      });
      console.log('SERVICES', services_list);
    } catch (err: any) {
      console.error('SERVICES', err);
    }
  };
  // FIX: the code below is an absolute dogshit
  // but we don't have enough time to clean code it xD
  const service_action_prompt = async (service: string) => {
    const should_delete = await ask(
      'What action do you want to take? If you want none just click edit and click cancel.',
      {
        title: 'Select Action',
        okLabel: 'Delete',
        cancelLabel: 'Edit',
      },
    );
    // INFO: handle delete case
    if (should_delete) {
      const confirmed = await confirm(
        'This action cannot be undone, please click OK if you want to proceed.',
        `Are you sure you want to delete ${service}?`,
      );
      if (confirmed) {
        // TODO: implement delete function
        try {
          const resp: string = await invoke('delete_service', { service });
          toast.success(resp);
          // INFO: why tf arr.remove or arr.popAt does not exists?
          services_list.splice(services_list.indexOf(service), 1);
          services_list = services_list;
        } catch (err: any) {
          toast.error(err);
        }
      }
      return;
    }
    // INFO: handle edit case
    let edited_service = prompt('Please enter new service name.', service);
    if (!edited_service) {
      return;
    }
    let edited_service_cleaned = edited_service.trim();
    if (!edited_service_cleaned.length) {
      toast.error('Service must be not be all whitespaces.');
      return;
    }
    if (edited_service_cleaned.length < 5) {
      toast.error('Service must be at least 5 characters.');
      return;
    }
    if (edited_service_cleaned === service) {
      toast.error('No change was made.');
      return;
    }
    // INFO: implement edit function
    try {
      const resp: string = await invoke('edit_service', {
        target: service,
        update: edited_service_cleaned,
      });
      toast.success(resp);
      services_list[services_list.indexOf(service)] = edited_service_cleaned;
    } catch (err: any) {
      toast.error(err);
    }
  };

  onMount(async () => {
    if (!$page.data.session) {
      toast.error('Please login first.');
      await goto('/login');
    }
    await load_services();
    try {
      const configs: ConfigData[] = await invoke('get_configs');
      configs.forEach((config) => {
        if (config.name === 'max_negative_feedback') {
          max_negative_feedback = parseInt(config.value);
        } else if (config.name === 'email_recipient') {
          email_recipient = config.value;
        } else if (config.name === 'office_name') {
          office_name = config.value;
        } else if (config.name === 'enable_consent_screen') {
          enable_consent_screen = config.value;
        } else if (config.name === 'emotion_weight') {
          emotion_weight = parseFloat(config.value);
        }
      });
      console.log('CONFIGS', configs);
    } catch (err: any) {
      console.error('CONFIGS', err);
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
                >Minimum Consecutive Negative Reviews for Alert</span
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
          <label class="form-control w-full">
            <div class="label">
              <span class="label-text">Office Name</span>
            </div>
            <input
              type="email"
              class="input input-bordered w-full"
              bind:value={office_name}
            />
            <div class="label">
              <span class="label-text-alt"
                >The name of the office in which this system is deployed.</span
              >
            </div>
          </label>
          <label class="form-control w-full">
            <div class="label">
              <span class="label-text">Emotion Weight</span>
            </div>
            <input
              type="number"
              max="0.99"
              min="0.01"
              step="0.01"
              class="input input-bordered w-full"
              bind:value={emotion_weight}
              bind:this={emotion_weight_input}
            />
            <div class="label">
              <span class="label-text-alt"
                >Specifies how much should emotion affect the final rating.</span
              >
            </div>
          </label>
          <div class="label">
            <span class="label-text">Services Offered</span>
          </div>
          <div class="flex flex-row gap-1">
            <ul class="menu bg-base-200 rounded-box rounded-r-none w-full">
              <li>
                <details>
                  <summary>List of Services Offered</summary>
                  <ul>
                    {#each services_list as service}
                      <!-- NOTE: idc about a11y on this case -->
                      <!-- svelte-ignore a11y-missing-attribute -->
                      <!-- svelte-ignore a11y-click-events-have-key-events -->
                      <!-- svelte-ignore a11y-no-static-element-interactions -->
                      <li>
                        <a
                          on:click={() => {
                            service_action_prompt(service);
                          }}>{service}</a
                        >
                      </li>
                    {/each}
                    <li></li>
                  </ul>
                </details>
              </li>
            </ul>
            <button
              class="btn btn-primary rounded-l-none min-h-[52px]"
              on:click={add_service}>Add Service</button
            >
          </div>
          <div class="label">
            <span class="label-text-alt"
              >The services offered by this office.</span
            >
          </div>
          <label class="form-control w-full">
            <div class="label">
              <span class="label-text">Consent Screen</span>
            </div>
            <div class="flex flex-inline">
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="enable_consent_screen"
                  bind:group={enable_consent_screen}
                  value="true"
                  class="radio"
                />
                <span class="label-text ml-2">On</span>
              </label>
              <label class="label cursor-pointer">
                <input
                  type="radio"
                  name="enable_consent_screen"
                  bind:group={enable_consent_screen}
                  value="false"
                  class="radio"
                />
                <span class="label-text ml-2">Off</span>
              </label>
            </div>
            <div class="label">
              <span class="label-text-alt"
                >Show a consent message prompt before the actual feedback page.</span
              >
            </div>
          </label>
          <button class="btn btn-primary mt-2 w-20 mb-20" on:click={save}
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
