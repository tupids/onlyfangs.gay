<script lang="ts">
  import { page } from '$app/stores';
  import { getContext } from 'svelte';
  import ApplicationForm from '../../../components/ApplicationForm.svelte';
  import type { DefinedQueryObserverResult } from '@tanstack/svelte-query';
  import type { PR } from '../../types';
  const prsQuery = getContext<DefinedQueryObserverResult<PR[], Error>>('prs');
  const applicationId = $page.params.id;
  const isNew = applicationId === 'new';

  //  Valid if no existing applications, is the users application, or is the new application
  let isValidApplication = $derived(
    isNew || (prsQuery.data?.some((pr: PR) => pr.id.toString() === applicationId) ?? false),
  );
</script>

{#if !isValidApplication}
  <p>No access</p>
{:else}
  <div class="container">
    <h1>{isNew ? 'New Application' : `Application #${applicationId}`}</h1>
    <ApplicationForm {applicationId} isEditable={isNew} />
  </div>
{/if}

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
