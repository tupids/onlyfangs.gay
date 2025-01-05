<script lang="ts">
  import { createQuery } from '@tanstack/svelte-query';
  const owner = 'ScuffleCloud';
  const repo = 'landing';

  type PR = {
    id: number;
    title: string;
    headRef: string;
    user: string;
  };

  let searchQuery = $state('');

  const prsQuery = createQuery<PR[]>({
    queryKey: ['prs'],
    initialData: [],
    queryFn: async () => {
      const response = await fetch(`https://api.github.com/repos/${owner}/${repo}/pulls?state=all`);
      const prs: PR[] = await response.json();

      return prs.map((pr: any) => ({
        id: pr.id,
        title: pr.title,
        headRef: pr.head.label,
        user: pr.user?.login || 'Unassigned',
      }));
    },
  });

</script>

<div class="container">
  <h1>My applications</h1>
  {#if $prsQuery.isLoading}
    <div class="loading">Loading...</div>
  {:else if $prsQuery.isError}
    <div class="error">Error: {$prsQuery.error.message}</div>
  {:else}
    <table>
      <thead>
        <tr>
          {#each ['id', 'title', 'headRef', 'user'] as header}
            <th
            >
              {header.charAt(0).toUpperCase() + header.slice(1)}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each $prsQuery.data as pr (pr.id)}
          <tr>
            <td>{pr.id}</td>
            <td>{pr.title}</td>
            <td>{pr.headRef}</td>
            <td>{pr.user}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .container {
    padding: 200px;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 20px;
  }

  th,
  td {
    padding: 20px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }

  tr:nth-child(even) {
    background-color: rgb(123, 34, 34);
  }

  .loading,
  .error {
    padding: 20px;
    text-align: center;
  }

  .error {
    color: red;
  }
</style>
