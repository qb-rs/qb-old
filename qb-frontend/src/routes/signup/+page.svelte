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
  export let cpassword: string;
</script>

<main>
  <form>
    <h1>Create your account</h1>

    <p>
      Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum
      sint consectetur cupidatat.
    </p>

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

    <div class="field" class:content={cpassword}>
      <input
        name="cpassword"
        id="cpassword"
        type="cpassword"
        bind:value={cpassword}
      />
      <label for="password">Confirm password</label>
    </div>

    <button>Continue</button>

    <p class="fnote">
      Already have an account? <a class="link" href="/signin">Sign in</a>
    </p>
  </form>
</main>

<style>
  main {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
  }

  form {
    display: flex;
    flex-direction: column;
    row-gap: 20px;
    width: 100%;
    max-width: 450px;
    padding: 40px 40px;
    background-color: white;
    border-radius: 20px;
    border: 1px solid rgba(0, 0, 0, 0.1);
  }
</style>
