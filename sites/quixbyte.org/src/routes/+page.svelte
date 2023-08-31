<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<!-- ████████████████████████████████████████████████ -->
<!-- █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█ -->
<!-- █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█ -->
<!-- ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀ -->
<!-- https://github.com/QuixByte/qb/blob/main/LICENSE -->

<!-- (c) Copyright 2023 The QuixByte Authors -->

<script lang="ts">
    import Fa from "svelte-fa";
    import {
        faServer,
        faCode,
        faGaugeHigh,
    } from "@fortawesome/free-solid-svg-icons";
    import { faDiscord, faGithub } from "@fortawesome/free-brands-svg-icons";
</script>

<svelte:head>
    <title>QuixByte - Sync with ease</title>
</svelte:head>

<section class="hero">
    <img src="/quixbyte_full.svg" alt="QuixByte" />
    <p>
        QuixByte is an opensource file service, which allows users to quickly
        synchronize and backup their files across multiple devices.
    </p>
    <ul>
        <li>
            <a href="https://github.com/QuixByte/qb" class="button"
                ><Fa icon={faGithub} />&nbsp; Github</a
            >
        </li>
        <li>
            <a href="https://discord.gg/Cq2UpzeTnm" class="button"
                ><Fa icon={faDiscord} />&nbsp; Discord</a
            >
        </li>
    </ul>
</section>

<hr />

<section>
    <span>&gt; bleeding edge tech stack</span>
    <h1>Performance to an extreme.</h1>

    <p>
        QuixByte is truly fast. This is because we have chosen to go with a
        modern tech stack.
    </p>

    <ul>
        <li>
            <a href="/" class="button"
                ><Fa icon={faGaugeHigh} />&nbsp; Tech stack</a
            >
        </li>
    </ul>
</section>

<hr />

<section>
    <span>&gt; file encryption + e2e file sync</span>
    <h1>Built with Privacy in mind.</h1>

    <p>
        Thanks to the no-third-party model and the selfhostability of QuixByte,
        your data is private by default.
    </p>

    <ul>
        <li>
            <a href="https://github.com/QuixByte/qb" class="button"
                ><Fa icon={faCode} />&nbsp; Source code</a
            >
        </li>
    </ul>
</section>

<hr />

<section>
    <span>&gt; your data - your server</span>
    <h1>Selfhostable with ease.</h1>

    <p>
        Running QuixByte on your own server only requires one command to work
        thanks to docker compose.
    </p>

    <ul>
        <li>
            <a href="/" class="button"
                ><Fa icon={faServer} />&nbsp; Setup instance</a
            >
        </li>
    </ul>
</section>

<hr />

<footer>
    <p>
        This page is under active development.
        <a href="https://github.com/QuixByte/qb/issues" class="link"
            >Report issue</a
        >
    </p>

    <p>
        &copy; Copyright 2023 The QuixByte Authors |
        <a href="https://github.com/QuixByte/qb/blob/main/LICENSE" class="link"
            >LICENSE</a
        >
    </p>
</footer>

<style>
    .hero img {
        height: 6rem;
        font-family: "Teko", sans-serif;
        font-size: 5rem;
        margin-bottom: 20px;
    }

    .hero {
        margin-top: 4rem;
    }

    span {
        display: inline-block;
        font-size: 32px;
        font-family: "Handjet", cursive;
        color: blue;
        margin-bottom: 0.3em;
    }

    section {
        margin-top: 8rem;
        margin-bottom: 8rem;
    }

    section p {
        font-size: 20px;
        max-width: 560px;
    }

    section ul,
    .hero ul {
        display: flex;
        list-style: none;
        column-gap: 10px;

        margin-top: 40px;
    }

    footer {
        padding-top: 3rem;
        font-size: 14px;
    }

    @media (max-width: 480px) {
        span {
            font-size: 30px;
            font-size: 6vw;
        }

        section h1 {
            font-size: 8vw;
        }

        section p {
            font-size: 17px;
        }

        .button {
            font-size: 17px;
        }

        .hero img {
            max-width: 90%;
        }

        footer {
            font-size: 12px;
            text-align: center;
        }
    }

    hr {
        border: none;
        border-top: 1px solid rgba(0, 0, 0, 0.1);
    }
</style>
