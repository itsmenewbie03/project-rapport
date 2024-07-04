<script lang="ts">
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";

  import { DataHandler, Datatable, Th, ThFilter } from "@vincjo/datatables";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { history_parser } from "$lib/parser";
  import { invoke } from "@tauri-apps/api/tauri";
  import toast from "svelte-french-toast";
  import ReportDetails from "$components/ReportDetails.svelte";
  let empty_any_arr: any[] = [];
  let handler = new DataHandler(empty_any_arr, { rowsPerPage: 10 });
  let rows = handler.getRows();
  let loaded: boolean = false;

  // INFO: modal stuff
  let modal_handle: any;
  let message: string = "{}";

  const view_details = (id: string) => {
    // INFO: shut up ts, we know what we are doing xD
    // and why do I need $ prefix here???
    let info = $rows.find((row) => row.id === id);
    message = JSON.stringify(info);
    modal_handle.show();
  };

  onMount(async () => {
    if (!$page.data.session) {
      toast.error("Please login first.");
      await goto("/login");
    }
    const hybrid_feedback_data: any = await invoke("get_feedbacks", {
      class: "hybrid",
    });
    const parsed_data = history_parser(hybrid_feedback_data);
    console.log(parsed_data);
    handler.setRows(parsed_data);
    // TEST: for aesthetics we will delay the load for a second xD
    setTimeout(() => {
      loaded = true;
    }, 1000);
  });
</script>

<UserLayout>
  <ReportDetails bind:this={modal_handle} {message} />
  <div class="flex flex-col h-[calc(100vh-144px)]">
    {#if loaded}
      {#if $page.data.session}
        <div class="px-14 py-4">
          <Datatable {handler} class="pb-14">
            <table class="table">
              <thead>
                <tr>
                  <Th {handler} orderBy="id">ID</Th>
                  <Th {handler} orderBy="office_name">Office</Th>
                  <Th {handler} orderBy="mean_rating">Overall Rating</Th>
                  <Th {handler} orderBy="mean_rating_percent"
                    >Overall Rating Percentage</Th
                  >
                  <Th {handler} orderBy="overall_emotion">Overall Emotion</Th>
                  <Th {handler} orderBy="overall_emotion_percent"
                    >Overall Emotion Percentage</Th
                  >
                  <Th {handler} orderBy="tag">Tag</Th>
                  <th></th>
                </tr>
                <tr>
                  <ThFilter {handler} filterBy="id" />
                  <ThFilter {handler} filterBy="office_name" />
                  <ThFilter {handler} filterBy="mean_rating" />
                  <ThFilter {handler} filterBy="mean_rating_percent" />
                  <ThFilter {handler} filterBy="overall_emotion" />
                  <ThFilter {handler} filterBy="overall_emotion_percent" />
                  <ThFilter {handler} filterBy="tag" />
                </tr>
              </thead>
              <tbody>
                {#each $rows as row}
                  <tr>
                    <td>{row.id}</td>
                    <td>{row.office_name}</td>
                    <td>{row.mean_rating}</td>
                    <td>{row.mean_rating_percent}%</td>
                    <td>{row.overall_emotion}</td>
                    <td>{row.overall_emotion_percent}%</td>
                    <td>{row.tag}</td>
                    <td>
                      <button
                        class="btn btn-primary btn-xs"
                        on:click={() => {
                          view_details(row.id);
                        }}>View</button
                      >
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </Datatable>
        </div>
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
