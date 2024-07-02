<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import toast from "svelte-french-toast";
  const generate = async () => {
    try {
      const res: string = await invoke("generate_report");
      toast.success(res);
    } catch (err: any) {
      toast.error(err);
    }
  };
</script>

<div class="card bg-base-200 w-full max-w-md shadow-xl bordered mt-6">
  <div class="card-body">
    <h2 class="card-title">Report Generator</h2>
    <label class="form-control w-full">
      <div class="label">
        <span class="label-text">Start Date</span>
      </div>
      <input
        type="text"
        class="input input-bordered w-full"
        value="FILTER NOT IMPLEMENTED!"
        readonly
      />
    </label>
    <label class="form-control w-full">
      <div class="label">
        <span class="label-text">End Date</span>
      </div>
      <input
        type="text"
        class="input input-bordered w-full"
        value="FILTER NOT IMPLEMENTED!"
        readonly
      />
    </label>
    <div class="card-actions justify-end mt-2">
      <button class="btn btn-primary" on:click={generate}>Generate</button>
    </div>
  </div>
</div>
