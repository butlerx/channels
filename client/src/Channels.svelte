<script lang="ts">
  import { messages, subscribe, send } from './store.ts';

  const urlParams = new URLSearchParams(window.location.search);
  let channels: string[] = urlParams.getAll('channels').sort();

  channels.forEach(subscribe);

  let channel;
  function subscribeToChannel(event: Event) {
    if (!channels.includes(channel)) {
      subscribe(channel);
      channels.push(channel);
      channels = [...channels].sort();
      urlParams.append('channels', channel);
      window.history.replaceState({}, '', `${location.pathname}?${urlParams}`);
    }
    channel = '';
  }

  function getUrl(channel: string) {
    return `${window.location.origin}/?channels=${channel}`;
  }
</script>

<style type="text/scss">
  .channels {
    grid-area: r;
    box-sizing: border-box;
    padding: 10px;
    background-color: var(--main-color);
    height: 100%;

    ul {
      list-style-type: none;
      padding: 0;
      overflow: scoll;

      a {
        color: white;
        font-weight: 600;
        text-decoration: none;
      }
    }

    li {
      margin: 10px 0;
    }

    h3 {
      margin: 5px 0;
      color: var(--secondary-color);
    }

    .channel {
      a {
        color: var(--secondary-text-color);
        font-weight: 600;
        text-decoration: none;
      }
    }
  }

  .new_channel {
    padding: 0 5px;
    background: var(--secondary-color);
    color: var(--main-text-color);
    grid-area: n;

    form {
      height: 100%;
      display: flex;
      justify-content: space-between;
      align-items: center;

      input {
        background: var(--secondary-color);
        border: 0;
        width: 80%;

        &::placeholder {
          color: var(--main-text-color);
          font-weight: 200;
        }

        &:focus {
          outline-width: 0;
        }
      }

      button {
        border: 0;
        background: var(--secondary-color);
        color: var(--main-text-color);
        border: 0;
        cursor: pointer;
      }
    }
  }
</style>

<div class="channels">
  <ul>
    <h3><a href="{window.location.origin}">Your channels:</a></h3>
    {#each channels as channel}
      <li class="channel"><a href="{getUrl(channel)}">#{channel}</a></li>
    {/each}
  </ul>
</div>
<div class="new_channel">
  <form on:submit|preventDefault="{subscribeToChannel}">
    <input type="text" placeholder="Subscribe to a Channel" required bind:value="{channel}" />
    <button type="submit">+</button>
  </form>
</div>
