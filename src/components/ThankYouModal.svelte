<script lang="ts">
  let handle: HTMLDialogElement;

  export const hide = () => {
    handle.classList.remove('modal-open');
  };

  export const show = () => {
    handle.classList.add('modal-open');
  };

  export let message: string;
</script>

<dialog id="confirm_model" class="backdrop-blur-md modal" bind:this={handle}>
  <div class="modal-box">
    <h3 class="font-bold text-lg">Thank You For Your Feedback!</h3>
    <p class="py-4">{message}</p>
  </div>
</dialog>
