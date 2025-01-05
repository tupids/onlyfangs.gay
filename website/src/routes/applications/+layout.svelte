<script lang="ts">
  import { createQuery } from '@tanstack/svelte-query';
  import { setContext } from 'svelte';
  import type { DefinedCreateQueryResult } from '@tanstack/svelte-query';
  import type { PR } from '../types';

  let { children } = $props();
  const prsQuery = createQuery<PR[]>({
    queryKey: ['prs'],
    enabled: true,
    initialData: [],
    queryFn: async () => {
      const response = await fetch(
        // CHANGE THIS TO BE THE USERS OWN APPLICATIONS
        `https://api.github.com/repos/ScuffleCloud/landing/pulls?state=all`,
      );
      const prs: PR[] = await response.json();

      return prs.map((pr: any) => ({
        id: pr.id,
        title: pr.title,
        headRef: pr.head.label,
        user: pr.user?.login || 'Unassigned',
      }));
    },
  });

  setContext<DefinedCreateQueryResult<PR[], Error>>('prs', prsQuery);
</script>

{#if !$prsQuery.isFetched}
  <div class="spinner-container">
    <div class="spinner"></div>
  </div>
{:else if $prsQuery.error}
  <p>Error: {$prsQuery.error.message}</p>
{:else}
  {@render children()}
{/if}

<style>
  .spinner-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 4px solid #e0e0e0;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
