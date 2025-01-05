<script lang="ts">
  import { page } from '$app/stores';
  import ApplicationForm from '../../../components/ApplicationForm.svelte';

  const applicationId = $page.params.id;
  const isNew = applicationId === 'new';
</script>

<div class="container">
  <h1>{isNew ? 'New Application' : `Application #${applicationId}`}</h1>
  <ApplicationForm {applicationId} isEditable={isNew} />
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
</style>
