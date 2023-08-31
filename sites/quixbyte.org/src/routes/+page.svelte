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

<section class="hero">
    <h1>QuixByte</h1>
    <p>Blazingly fast alternative to NextCloud&trade; & co</p>
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
        Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint
        cillum sint consectetur cupidatat.
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
        Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint
        cillum sint consectetur cupidatat.
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
        Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint
        cillum sint consectetur cupidatat.
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
    .hero h1 {
        font-size: 5rem;
        font-family: "Teko", sans-serif;
    }

    .hero p {
        font-size: 1.5rem;
    }

    .hero {
        margin-top: 4rem;
    }

    span {
        display: inline-block;
        font-size: 36px;
        font-family: "Handjet", cursive;
        color: blue;
        margin-bottom: 0.3em;
    }

    section {
        margin-top: 8rem;
        margin-bottom: 8rem;
        font-size: 1.4rem;
    }

    section h1 {
        margin-bottom: 2rem;
    }

    section ul,
    .hero ul {
        display: flex;
        list-style: none;
        column-gap: 10px;

        margin-top: 2rem;
    }

    footer {
        padding-top: 3rem;
        font-size: 14px;
        font-family: "Inter", sans-serif;
    }

    @media (max-width: 500px) {
        span {
            font-size: 30px;
        }

        section h1 {
            font-size: 40px;
        }

        section p {
            font-size: 20px;
        }

        .button {
            font-size: 18px;
        }

        .hero {
            text-align: center;
        }

        .hero ul {
            justify-content: center;
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
