<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<!-- ████████████████████████████████████████████████ -->
<!-- █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█ -->
<!-- █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█ -->
<!-- ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀ -->
<!-- https://github.com/QuixByte/qb/blob/main/LICENSE -->

<!-- (c) Copyright 2023 The QuixByte Authors -->

<script lang="ts">
  export let name: string;
  export let password: string;
</script>

<main>
  <form>
    <img src="/quixbyte_full.svg" alt="QuixByte" />

    <p>Please sign in to continue.</p>

    <br />

    <div class="field" class:content={name}>
      <input name="name" id="name" bind:value={name} />
      <label for="name">Name</label>
    </div>

    <div class="field" class:content={password}>
      <input
        name="password"
        id="password"
        type="password"
        bind:value={password}
      />
      <label for="password">Password</label>
    </div>

    <button>Continue</button>

    <br />

    <p class="fnote">
      Don't have an account? <a class="link" href="/signup">Sign up</a>
    </p>
  </form>
</main>

<style>
  main {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 20px 0;
    background-image: url("/quixbyte_bg_light.svg");
    background-size: 180px;
  }

  br {
    margin: 10px 0;
  }

  button {
    margin-top: 10px;
  }

  form {
    display: flex;
    flex-direction: column;
    row-gap: 10px;
    width: 100%;
    max-width: 450px;
    padding: 40px 60px;
    background-color: white;
    border-radius: 20px;
    box-shadow: 0px 0px 40px 20px rgba(0, 0, 0, 0.05);
  }

  img {
    width: 100%;
  }

  @media (max-width: 450px) {
    form {
      box-shadow: none;
      padding: 40px 0;
      width: 85%;
    }

    main {
      background-image: none;
      align-items: start;
      padding: 60px 0;
    }
  }
</style>
