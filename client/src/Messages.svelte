<script lang="ts">
  import Message from './Message.svelte';
  import { messages, channels, send } from './store.ts';

  let channel;
  let msg;
  function onSendMessage() {
    if (channel && msg) {
      send(channel, msg);
      msg = '';
    }
  }
</script>

<style type="text/scss">
  .messages {
    grid-area: m;
    box-sizing: border-box;
    padding-left: 6px;
    width: 100%;
    height: 100%;
    overflow: scroll;
    background: var(--secondary-color);
  }

  form {
    background: var(--send-message-form);
    display: flex;
    grid-area: f;

    input {
      width: 100%;
      padding: 15px 10px;
      margin: 0;
      border-style: none;
      background: var(--send-message-form);
      font-weight: 200;

      &:focus {
        outline-width: 0;
      }

      &::placeholder {
        color: var(--main-text-color);
      }
    }

    button,
    select {
      border: 0;
      background: var(--send-message-form);
      color: var(--main-text-color);
      cursor: pointer;
    }
  }
</style>

<div class="messages">
  {#each $messages as message}
    <Message message="{message}" />
  {/each}
</div>
<form on:submit|preventDefault="{onSendMessage}">
  <select bind:value="{channel}" disabled="{!$channels}">
    {#each $channels as channel}
      <option value="{channel}">{channel}</option>
    {/each}
  </select>
  <input
    placeholder="Type your message and hit ENTER"
    type="text"
    bind:value="{msg}"
    disabled="{!channel}" />
  <button type="submit" disabled="{!channel}"> &#8617;</button>
</form>
