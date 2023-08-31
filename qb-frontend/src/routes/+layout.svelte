<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<!-- ████████████████████████████████████████████████ -->
<!-- █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█ -->
<!-- █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█ -->
<!-- ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀ -->
<!-- https://github.com/QuixByte/qb/blob/main/LICENSE -->

<!-- (c) Copyright 2023 The QuixByte Authors -->

<script lang="ts">
  import "../app.css";

  import "@fontsource/inter/latin-400.css";
  import "@fontsource/inter/latin-900.css";
  import "@fontsource/inter/latin-700.css";
</script>

<slot />

<p>Powered by <a class="link" href="https://quixbyte.tech">QuixByte</a></p>

<style>
  p {
    position: absolute;
    left: 10px;
    bottom: 5px;
    font-size: 14px;
  }
</style>
