<script lang="ts">
  export let applicationId: string;
  export let isEditable: boolean = false;

  let formData = {
    reason: '',
    support_clip_url: '',
  };

  function handleSubmit() {
    console.log('Form submitted:', {
      applicationId,
      ...formData,
    });
  }
</script>

<form class="application-form" on:submit|preventDefault={handleSubmit}>
  <div class="form-group">
    <label for="reason">Reason</label>
    {#if isEditable}
      <textarea id="reason" bind:value={formData.reason} required rows="4" placeholder="Reason..."
      ></textarea>
    {:else}
      <div class="readonly-field">{formData.reason}</div>
    {/if}
  </div>

  <div class="form-group">
    <label for="clip">Support Clip/Youtube URL</label>
    {#if isEditable}
      <input
        type="url"
        id="clip"
        bind:value={formData.support_clip_url}
        placeholder="https://clips.twitch.tv/..."
        required
      />
    {:else}
      <div class="readonly-field">{formData.support_clip_url}</div>
    {/if}
  </div>

  {#if isEditable}
    <button type="submit">Submit Application</button>
  {/if}
</form>

<style>
  .application-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-weight: bold;
  }

  textarea,
  input {
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 0.25rem;
    font-size: 1rem;
  }

  textarea {
    resize: vertical;
    min-height: 100px;
  }

  button {
    padding: 0.75rem 1.5rem;
    background-color: #4caf50;
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    font-size: 1rem;
  }

  button:hover {
    background-color: #45a049;
  }

  .readonly-field {
    padding: 0.5rem;
    background-color: #f5f5f5;
    border-radius: 0.25rem;
    min-height: 1.5rem;
  }
</style>
