<script lang="ts">
  import toast from "svelte-french-toast";
  import { invoke } from "@tauri-apps/api/tauri";

  let target: string = "";
  let message: string = "";
  let count: number = 0;
  let sent_count: number = 0;
  let failed_count: number = 0;
  let random_messages: boolean = false;

  const clear = (event: Event) => {
    event.preventDefault();
    target = "";
    message = "";
    sent_count = 0;
  };

  const validate_url = (url: string) => {
    // INFO: we're parsing the url input
    // this returns the user name if the url is valid
    return /(?<=https:\/\/ngl\.link\/).*/.exec(url)?.[0];
  };

  const start = async (event: Event) => {
    event.preventDefault();
    if (target === "") {
      toast.error("Please specify a target!");
      return;
    }
    if (!random_messages && message === "") {
      toast.error("Please fill all the fields!");
      return;
    }
    if (!count) {
      toast.error("Count must be at least 1!");
      return;
    }
    const username = validate_url(target);
    if (!username) {
      toast.error("Invalid ngl url!");
      return;
    }
    // INFO: we can now call rust here using tauri
    // for now we will just log what we have
    console.log({ username, message, count, random_messages });
    const resp = await invoke("spam", {
      target: username,
      count: count,
      random: random_messages,
      message: message,
    });
    // TODO: we just tell TS to shut up coz why not?
    // JK I will refactor this "later"
    // @ts-ignore
    const { message: msg, success, failed } = resp;
    sent_count = parseInt(success);
    failed_count = parseInt(failed);
    toast(msg, { icon: "🛈" });
  };

  const toggle = (event: Event) => {
    random_messages = (<HTMLInputElement>event.target).checked;
  };
</script>

<div class="flex w-full justify-center">
  <div class="card w-96 bg-base-100 shadow-xl bordered">
    <div class="card-body">
      <h2 class="text-4xl card-title">Let's get started!</h2>
      <label class="form-control w-full max-w-xs">
        <div class="label">
          <span class="label-text">Who to annoy?</span>
        </div>
        <input
          type="text"
          placeholder="Enter ngl url ..."
          class="input input-bordered w-full max-w-xs"
          bind:value={target}
        />
      </label>
      <label class="label cursor-pointer">
        <span class="label-text">Random messages?</span>
        <input
          type="checkbox"
          class="toggle toggle-primary"
          on:click={toggle}
          checked={random_messages}
        />
      </label>
      <!-- NOTE: We only show the message field if the user doesn't want random messages -->
      {#if !random_messages}
        <label class="form-control w-full max-w-xs">
          <div class="label">
            <span class="label-text">What to send?</span>
          </div>
          <input
            type="text"
            placeholder="Type your message here ..."
            class="input input-bordered w-full max-w-xs"
            bind:value={message}
          />
        </label>
      {/if}

      <label class="form-control w-full max-w-xs">
        <div class="label">
          <span class="label-text">How many?</span>
        </div>
        <input
          type="number"
          max="20"
          min="1"
          placeholder="Type your message here ..."
          class="input input-bordered w-full max-w-xs"
          bind:value={count}
        />
      </label>
      <p class="text-xs text-center text-primary-500">
        <span class="text-success">{sent_count} messages sent </span>
        ┊
        <span class="text-error">{failed_count} failed</span>
      </p>
      <div class="card-actions justify-end">
        <button class="btn" on:click={clear}>Clear Form</button>
        <button class="btn btn-primary" on:click={start}>Start Spamming</button>
      </div>
    </div>
  </div>
</div>
