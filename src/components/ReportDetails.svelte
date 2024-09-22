<script lang="ts">
  import { beforeUpdate, onMount } from 'svelte';

  let handle: HTMLDialogElement;
  const pretty_key = (key: string) => {
    let dont_capitalize = ['for', 'and'];
    return key
      .replaceAll('_', ' ')
      .split(' ')
      .map((word) => {
        if (dont_capitalize.includes(word)) return word;
        return word.charAt(0).toUpperCase() + word.slice(1);
      })
      .join(' ');
  };
  export const hide = () => {
    handle.classList.remove('modal-open');
  };

  export const show = () => {
    handle.classList.add('modal-open');
  };
  export let message: string;
  let data: any = {};
  beforeUpdate(() => {
    data = JSON.parse(message);
    console.log('PARSED', data);
  });
</script>

<dialog id="confirm_model" class="backdrop-blur-md modal" bind:this={handle}>
  <div class="modal-box">
    <h3 class="font-bold text-lg">Feedback Details</h3>
    <table class="table">
      <thead>
        <tr>
          <th>Aspect</th>
          <th>Rating</th>
          <th>Emotion</th>
        </tr>
      </thead>
      <tbody>
        {#if Object.keys(data).length > 0}
          {#each Object.keys(data.feedback_data) as aspect}
            <tr>
              <td>{pretty_key(aspect)}</td>
              <td>{data.feedback_data[aspect]}</td>
              <td>{data.emotion_data[aspect]}</td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
    <div class="modal-action">
      <button
        class="btn btn-primary"
        on:click={() => {
          hide();
        }}>Close</button
      >
    </div>
  </div>
</dialog>
