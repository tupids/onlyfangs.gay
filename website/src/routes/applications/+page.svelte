<script lang="ts">
  import { goto } from '$app/navigation';
  // import type { DefinedCreateQueryResult } from '@tanstack/svelte-query';
  // import { getContext } from 'svelte';
  // const prsQuery = getContext<DefinedCreateQueryResult<PR[], Error>>('prs');

  const handleRowClick = (id: string) => {
    goto(`/applications/${id}`);
  };
</script>

<div class="container">
  <!-- {#if !$prsQuery.isFetched}
    <div></div>
  {:else if $prsQuery.isFetched && $prsQuery.data.length === 0}
    <div class="start-application">
      <button>Start Application</button>
    </div>
  {:else if $prsQuery.isFetched && $prsQuery.data.length > 0}
    <h1>My applications</h1>
    <table>
      <thead>
        <tr>
          {#each ['id', 'title', 'headRef', 'user'] as header}
            <th>
              {header.charAt(0).toUpperCase() + header.slice(1)}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each $prsQuery.data as pr (pr.id)}
          <tr on:click={() => handleRowClick(pr.id.toString())} class="clickable-row">
            <td>{pr.id}</td>
            <td>{pr.title}</td>
            <td>{pr.headRef}</td>
            <td>{pr.user}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if} -->
</div>

<style>
  .container {
    padding: 200px;
  }

  /* .start-application {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 50vh;
  }

  button {
    padding: 1rem 2rem;
    font-size: 1.2rem;
    background-color: #4caf50;
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
  } */

  /* tr:nth-child(even) {
    background-color: rgb(123, 34, 34);
  }

  .clickable-row {
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .clickable-row:hover {
    background-color: rgba(0, 0, 0, 0.05);
  } */
</style>
