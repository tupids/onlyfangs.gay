<script lang="ts">
    import { page } from '$app/stores';
  
    let formData = {
      reason: '',
      support_clip_url: ''
    };
  
    const applicationId = $page.params.id;
  
    function handleSubmit() {
      console.log('Form submitted:', {
        applicationId,
        ...formData
      });
    }
  </script>
  
  <div class="container">
    <h1>Application #{applicationId}</h1>
    <form class="application-form" on:submit|preventDefault={handleSubmit}>
      <div class="form-group">
        <label for="reason">Reason</label>
        <textarea 
          id="reason"
          bind:value={formData.reason}
          required
          rows="4"
          placeholder="Reason..."
        ></textarea>
      </div>
  
      <div class="form-group">
        <label for="clip">Support Clip/Youtube URL</label>
        <input 
          type="url"
          id="clip"
          bind:value={formData.support_clip_url}
          placeholder="https://clips.twitch.tv/..."
          required
        />
      </div>
      
      <button type="submit">Submit Application</button>
    </form>
  </div>
  
  <style>
    .container {
      padding: 5rem;
      max-width: 800px;
      margin: 0 auto;
      display: flex;
      flex-direction: column;
      gap: 2rem;
    }
  
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
  
    textarea, input {
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
      background-color: #4CAF50;
      color: white;
      border: none;
      border-radius: 0.25rem;
      cursor: pointer;
      font-size: 1rem;
    }
  
    button:hover {
      background-color: #45a049;
    }
  </style>
  