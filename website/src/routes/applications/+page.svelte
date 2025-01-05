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

//   REPLACE WITH APPLICATIONS QUERY
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
  {#if $prsQuery.isLoading}
    <div class="loading">Loading...</div>
  {:else if $prsQuery.isError}
    <div class="error">Error: {$prsQuery.error.message}</div>
  {:else if $prsQuery.data.length === 0}
    <div class="start-application">
      <button>Start Application</button>
    </div>
  {:else}
  <h1>My applications</h1>
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

  .start-application {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 50vh;
  }

  button {
    padding: 1rem 2rem;
    font-size: 1.2rem;
    background-color: #4CAF50;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  button:hover {
    background-color: #45a049;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 1rem;
  }

  th,
  td {
    padding: 1rem;
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
