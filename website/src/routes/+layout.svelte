<script lang="ts">
  import { browser } from '$app/environment';
  import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query';
  import '../styles/global.css';

  let { children } = $props();
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        enabled: browser,
      },
    },
  });
</script>

<div class="app">
  <QueryClientProvider client={queryClient}>
    <main>
      {@render children()}
    </main>
  </QueryClientProvider>
</div>
