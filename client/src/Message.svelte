<script lang="ts">
  export let message;

  import hljs from 'highlight.js/lib/highlight';
  import json from 'highlight.js/lib/languages/json';
  import plaintext from 'highlight.js/lib/languages/plaintext';
  import 'highlight.js/styles/monokai-sublime.css';

  hljs.registerLanguage('json', json);
  hljs.registerLanguage('plaintext', plaintext);

  function formatMsg(msg: string | object) {
    if (typeof msg === 'string') {
      return hljs.highlight('plaintext', msg).value;
    }
    return hljs.highlight('json', JSON.stringify(msg, undefined, 2)).value;
  }
</script>

<style type="text/scss">
  .message {
    display: flex;
    margin: 1rem;

    .meta {
      background: var(--secondary-text-color);
      border-radius: 1rem;
      display: flex;
      flex-direction: column;
      font-size: 0.8rem;
      justify-content: space-between;
      margin-right: 1rem;
      padding: 0.4rem;

      .channel {
        color: var(--main-text-color);
        font-weight: bold;
        opacity: 0.9;
      }
      .timestamp {
        align-self: flex-end;
      }
    }
    .text {
      display: flex;
      margin: 0;
      width: 100%;
      .hljs {
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
        width: 100%;
      }
    }
  }
</style>

<div class="message">
  <div class="meta">
    <div class="channel">#{message.channel}</div>
    <div class="timestamp">{message.timestamp}</div>
  </div>
  <pre class="text" on:click on:mouseover on:mouseenter on:mouseleave on:focus on:blur><code
      class="language-json hljs">{@html formatMsg(message.text)}</code></pre>
</div>
