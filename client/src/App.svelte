<script lang="ts">
  import Messages from './Messages.svelte';
  import Channels from './Channels.svelte';
</script>

<style type="text/scss">
  :root {
    --main-color: #5ea3d0;
    --secondary-color: white;
    --main-text-color: #3e5869;
    --secondary-text-color: #b0c7d6;
    --send-message-form: #f5f5f5;
  }

  :global(html, body) {
    font-family: Arial, Helvetica, sans-serif;
    height: 100%;
    margin: 0;
    padding: 0;
    font-weight: 200;
    color: #3e5869;
  }

  .app {
    display: grid;
    height: 100%;
    grid-template-columns: repeat(6, 1fr);
    grid-template-rows: 1fr 1fr 1fr 1fr 1fr 60px;
    grid-template-areas:
      'r m m m m m'
      'r m m m m m'
      'r m m m m m'
      'r m m m m m'
      'r m m m m m'
      'n f f f f f';
  }
</style>

<div class="app">
  <Channels />
  <Messages />
</div>
