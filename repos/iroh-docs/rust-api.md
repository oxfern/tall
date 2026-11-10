<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Peer-to-peer QUIC connections."><title>iroh - Rust</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-6b053e98.ttf.woff2,FiraSans-Italic-81dc35de.woff2,FiraSans-Regular-0fe48ade.woff2,FiraSans-MediumItalic-ccf7e434.woff2,FiraSans-Medium-e1aa3f0a.woff2,SourceCodePro-Regular-8badfe75.ttf.woff2,SourceCodePro-Semibold-aa29a496.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2"href="/-/rustdoc.static/${f}">`).join(""))</script><link rel="stylesheet" href="/-/rustdoc.static/normalize-9960930a.css"><link rel="stylesheet" href="/-/static/v/4ddc5de0b7e3fdcc/vendored.css" media="all" /><link rel="stylesheet" href="/-/rustdoc.static/rustdoc-009149e2.css"><meta name="rustdoc-vars" data-root-path="../" data-static-root-path="/-/rustdoc.static/" data-current-crate="iroh" data-themes="" data-resource-suffix="-20261020-1.101.0-nightly-19833af72" data-rustdoc-version="1.101.0-nightly (19833af72 2026-10-20)" data-channel="nightly" data-search-js="search-b167f383.js" data-stringdex-js="stringdex-b5109af9.js" data-settings-js="settings-9b351432.js" ><script src="/-/rustdoc.static/storage-41dd4d93.js"></script><script defer src="../crates-20261020-1.101.0-nightly-19833af72.js"></script><script defer src="/-/rustdoc.static/main-1814ead3.js"></script><noscript><link rel="stylesheet" href="/-/rustdoc.static/noscript-9f56287d.css"></noscript><link rel="alternate icon" type="image/png" href="/-/rustdoc.static/favicon-32x32-eab170b8.png"><link rel="icon" type="image/svg+xml" href="/-/rustdoc.static/favicon-044be391.svg"><link rel="stylesheet" href="/-/static/v/4ddc5de0b7e3fdcc/rustdoc-2025-08-20.css" media="all" /><link rel="stylesheet" href="/-/static/v/4ddc5de0b7e3fdcc/font-awesome.css" media="all" />

<link rel="search" href="/-/static/v/4ddc5de0b7e3fdcc/opensearch.xml" type="application/opensearchdescription+xml" title="Docs.rs" />

<script type="text/javascript">(function() {
    function applyTheme(theme) {
        if (theme) {
            document.documentElement.dataset.docsRsTheme = theme;
        }
    }

    window.addEventListener("storage", ev => {
        if (ev.key === "rustdoc-theme") {
            applyTheme(ev.newValue);
        }
    });

    // see ./storage-change-detection.html for details
    window.addEventListener("message", ev => {
        if (ev.data && ev.data.storage && ev.data.storage.key === "rustdoc-theme") {
            applyTheme(ev.data.storage.value);
        }
    });

    applyTheme(window.localStorage.getItem("rustdoc-theme"));
})();</script></head><body class="rustdoc-page">
<div class="nav-container">
    <div class="container">
        <div class="pure-menu pure-menu-horizontal" role="navigation" aria-label="Main navigation">
            <form action="/releases/search"
                  method="GET"
                  id="nav-search-form"
                  class="landing-search-form-nav  ">

                
                <a href="/" class="pure-menu-heading pure-menu-link docsrs-logo" aria-label="Docs.rs">
                    <span title="Docs.rs"><span class="fa fa-solid fa-cubes " aria-hidden="true"></span></span>
                    <span class="title">Docs.rs</span>
                </a><ul class="pure-menu-list">
    <script id="crate-metadata" type="application/json">
        
        {
            "name": "iroh",
            "version": "1.4.1"
        }
    </script><li class="pure-menu-item pure-menu-has-children crate-dropdown">
            <a href="#" class="pure-menu-link crate-name" title="p2p quic connections dialed by public key">
                <span class="fa fa-solid fa-cube " aria-hidden="true"></span>
                <span class="title">iroh-1.4.1</span>
            </a><div class="pure-menu-children package-details-menu">
                
                <ul class="pure-menu-list menu-item-divided">
                    <li class="pure-menu-heading" id="crate-title">
                        iroh 1.4.1
                        <span id="clipboard" class="svg-clipboard" title="Copy crate name and version information"></span>
                    </li><li class="pure-menu-item">
                        <a href="/iroh/1.4.1/iroh/" class="pure-menu-link description" id="permalink" data-fragment="retain" title="Get a link to this specific version"><span class="fa fa-solid fa-link " aria-hidden="true"></span> Permalink
                        </a>
                    </li><li class="pure-menu-item">
                        <a href="/crate/iroh/latest" class="pure-menu-link description" title="See iroh in docs.rs">
                            <span class="fa fa-solid fa-cube " aria-hidden="true"></span> Docs.rs crate page
                        </a>
                    </li><li class="pure-menu-item">
                            <span class="pure-menu-link description license"><span class="fa fa-solid fa-scale-unbalanced-flip " aria-hidden="true"></span>
                            <a href="https://spdx.org/licenses/MIT" class="pure-menu-sublink">MIT</a> OR <a href="https://spdx.org/licenses/Apache-2.0" class="pure-menu-sublink">Apache-2.0</a></span>
                        </li><li class="pure-menu-item">
                            <span class="pure-menu-link description" title="Built with rustc 1.101.0-nightly (19833af72 2026-10-20)">
                                <span class="fa fa-solid fa-gears " aria-hidden="true"></span> 21 October 2026
                            </span>
                        </li></ul>

                <div class="pure-g menu-item-divided">
                    <div class="pure-u-1-2 right-border">
                        <ul class="pure-menu-list">
                            <li class="pure-menu-heading">Links</li>

                            <li class="pure-menu-item">
                                    <a href="https://github.com/n0-computer/iroh" class="pure-menu-link">
                                        <span class="fa fa-solid fa-code-branch " aria-hidden="true"></span> Repository
                                    </a>
                                </li><li class="pure-menu-item">
                                <a href="https://crates.io/crates/iroh" class="pure-menu-link" title="See iroh in crates.io">
                                    <span class="fa fa-solid fa-cube " aria-hidden="true"></span> crates.io
                                </a>
                            </li>

                            
                            <li class="pure-menu-item">
                                <a href="/crate/iroh/latest/source/" title="Browse source of iroh-1.4.1" class="pure-menu-link">
                                    <span class="fa fa-solid fa-folder-open " aria-hidden="true"></span> Source
                                </a>
                            </li>
                        </ul>
                    </div><div class="pure-u-1-2">
                        <ul class="pure-menu-list" id="topbar-owners">
                            <li class="pure-menu-heading">Owners</li><li class="pure-menu-item">
                                    <a href="https://crates.io/users/dignifiedquire" class="pure-menu-link">
                                        <span class="fa fa-solid fa-user " aria-hidden="true"></span> dignifiedquire
                                    </a>
                                </li><li class="pure-menu-item">
                                    <a href="https://crates.io/teams/github:n0-computer:iroh-publisher" class="pure-menu-link">
                                        <span class="fa fa-solid fa-user " aria-hidden="true"></span> github:n0-computer:iroh-publisher
                                    </a>
                                </li></ul>
                    </div>
                </div>

                <div class="pure-g menu-item-divided">
                    <div class="pure-u-1-2 right-border">
                        <ul class="pure-menu-list">
                            <li class="pure-menu-heading">Dependencies</li>

                            
                            <li class="pure-menu-item">
                                <div class="pure-menu pure-menu-scrollable sub-menu" tabindex="-1">
                                    <ul class="pure-menu-list">
                                        <li class="pure-menu-item"><a href="/axum/^0.8/" class="pure-menu-link">
                axum ^0.8
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/backon/^1.4/" class="pure-menu-link">
                backon ^1.4
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/blake3/^1.8.3/" class="pure-menu-link">
                blake3 ^1.8.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/bytes/^1.11/" class="pure-menu-link">
                bytes ^1.11
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/ctutils/^0.4.0/" class="pure-menu-link">
                ctutils ^0.4.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/data-encoding/^2.2/" class="pure-menu-link">
                data-encoding ^2.2
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/derive_more/^2.0.1/" class="pure-menu-link">
                derive_more ^2.0.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/ed25519-dalek/%3E=3.0.0-rc.0,%20%3C4.0.0/" class="pure-menu-link">
                ed25519-dalek >=3.0.0-rc.0, <4.0.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/futures-util/^0.3/" class="pure-menu-link">
                futures-util ^0.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/http/^1/" class="pure-menu-link">
                http ^1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/ipnet/^2/" class="pure-menu-link">
                ipnet ^2
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/iroh-base/^1.4.1/" class="pure-menu-link">
                iroh-base ^1.4.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/iroh-dns/^1.4.1/" class="pure-menu-link">
                iroh-dns ^1.4.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/iroh-metrics/^1.0.2/" class="pure-menu-link">
                iroh-metrics ^1.0.2
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/iroh-relay/^1.4.1/" class="pure-menu-link">
                iroh-relay ^1.4.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/n0-error/^1.0.0/" class="pure-menu-link">
                n0-error ^1.0.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/n0-future/^0.3/" class="pure-menu-link">
                n0-future ^0.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/n0-watcher/^1.0.0/" class="pure-menu-link">
                n0-watcher ^1.0.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/netwatch/^0.19.3/" class="pure-menu-link">
                netwatch ^0.19.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/noq/^1.4.0/" class="pure-menu-link">
                noq ^1.4.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/noq-proto/^1.4.0/" class="pure-menu-link">
                noq-proto ^1.4.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/noq-udp/^1.4.0/" class="pure-menu-link">
                noq-udp ^1.4.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/papaya/^0.2.3/" class="pure-menu-link">
                papaya ^0.2.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/pin-project/^1/" class="pure-menu-link">
                pin-project ^1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/portable-atomic/^1/" class="pure-menu-link">
                portable-atomic ^1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/rand/^0.10/" class="pure-menu-link">
                rand ^0.10
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/reqwest/^0.13/" class="pure-menu-link">
                reqwest ^0.13
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/rustc-hash/^2/" class="pure-menu-link">
                rustc-hash ^2
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/rustls/^0.23.46/" class="pure-menu-link">
                rustls ^0.23.46
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde/^1.0.219/" class="pure-menu-link">
                serde ^1.0.219
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/simple-dns/^0.12/" class="pure-menu-link">
                simple-dns ^0.12
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/smallvec/^1.11.1/" class="pure-menu-link">
                smallvec ^1.11.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/strum/^0.28/" class="pure-menu-link">
                strum ^0.28
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio/^1.44.1/" class="pure-menu-link">
                tokio ^1.44.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio-stream/^0.1.15/" class="pure-menu-link">
                tokio-stream ^0.1.15
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio-util/^0.7/" class="pure-menu-link">
                tokio-util ^0.7
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tracing/^0.1/" class="pure-menu-link">
                tracing ^0.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/url/^2.5/" class="pure-menu-link">
                url ^2.5
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/rustls-pki-types/^1.12/" class="pure-menu-link">
                rustls-pki-types ^1.12
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/chrono/^0.4.43/" class="pure-menu-link">
                chrono ^0.4.43
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/console_error_panic_hook/^0.1/" class="pure-menu-link">
                console_error_panic_hook ^0.1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/n0-error/^1.0.0/" class="pure-menu-link">
                n0-error ^1.0.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/postcard/^1.1.1/" class="pure-menu-link">
                postcard ^1.1.1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/rand_chacha/^0.10/" class="pure-menu-link">
                rand_chacha ^0.10
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/simple-dns/^0.12/" class="pure-menu-link">
                simple-dns ^0.12
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tracing-subscriber/^0.3/" class="pure-menu-link">
                tracing-subscriber ^0.3
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/cfg_aliases/^0.2.2/" class="pure-menu-link">
                cfg_aliases ^0.2.2
                
                    <i class="dependencies build">build</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/getrandom/^0.4/" class="pure-menu-link">
                getrandom ^0.4
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/time/^0.3/" class="pure-menu-link">
                time ^0.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/wasm-bindgen-futures/^0.4/" class="pure-menu-link">
                wasm-bindgen-futures ^0.4
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/wasm-bindgen-test/^0.3.62/" class="pure-menu-link">
                wasm-bindgen-test ^0.3.62
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/wasm-tracing/^2.1.0/" class="pure-menu-link">
                wasm-tracing ^2.1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/noq/^1.4.0/" class="pure-menu-link">
                noq ^1.4.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/portmapper/^0.19.3/" class="pure-menu-link">
                portmapper ^0.19.3
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio/^1/" class="pure-menu-link">
                tokio ^1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/assert_matches/^1.5.0/" class="pure-menu-link">
                assert_matches ^1.5.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/axum/^0.8/" class="pure-menu-link">
                axum ^0.8
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/clap/^4/" class="pure-menu-link">
                clap ^4
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/console/^0.16/" class="pure-menu-link">
                console ^0.16
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/indicatif/^0.18/" class="pure-menu-link">
                indicatif ^0.18
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/iroh-base/^1.4.1/" class="pure-menu-link">
                iroh-base ^1.4.1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/iroh-relay/^1.4.1/" class="pure-menu-link">
                iroh-relay ^1.4.1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/n0-tracing-test/^0.3/" class="pure-menu-link">
                n0-tracing-test ^0.3
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/parse-size/^1.1.0/" class="pure-menu-link">
                parse-size ^1.1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/pretty_assertions/^1.4/" class="pure-menu-link">
                pretty_assertions ^1.4
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/rand_chacha/^0.10/" class="pure-menu-link">
                rand_chacha ^0.10
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde_json/^1/" class="pure-menu-link">
                serde_json ^1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio/^1/" class="pure-menu-link">
                tokio ^1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tracing-subscriber/^0.3/" class="pure-menu-link">
                tracing-subscriber ^0.3
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/ctor/^1.0/" class="pure-menu-link">
                ctor ^1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/patchbay/^0.7/" class="pure-menu-link">
                patchbay ^0.7
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/testdir/^0.10/" class="pure-menu-link">
                testdir ^0.10
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li>
                                    </ul>
                                </div>
                            </li>
                        </ul>
                    </div>

                    <div class="pure-u-1-2">
                        <ul class="pure-menu-list">
                            <li class="pure-menu-heading">Versions</li>

                            <li class="pure-menu-item">
                                <div class="pure-menu pure-menu-scrollable sub-menu" id="releases-list" tabindex="-1" data-url="/crate/iroh/latest/menus/releases/iroh/">
                                    <span class="rotate"><span class="fa fa-solid fa-spinner " aria-hidden="true"></span></span>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>
                    
                    
                    <div class="pure-g">
                        <div class="pure-u-1">
                            <ul class="pure-menu-list">
                                <li>
                                    <a href="/crate/iroh/latest" class="pure-menu-link">
                                        <b>100%</b>
                                        of the crate is documented
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div></div>
        </li><li class="pure-menu-item pure-menu-has-children">
                <a href="#" class="pure-menu-link" aria-label="Platform">
                    <span class="fa fa-solid fa-gears " aria-hidden="true"></span>
                    <span class="title">Platform</span>
                </a>

                
                <ul class="pure-menu-children" id="platforms" data-url="/crate/iroh/latest/menus/platforms/iroh/"><li class="pure-menu-item">
            <a href="/crate/iroh/latest/target-redirect/iroh/" class="pure-menu-link" data-fragment="retain" rel="nofollow">x86_64-unknown-linux-gnu</a>
        </li></ul>
            </li><li class="pure-menu-item">
                <a href="/crate/iroh/latest/features" title="Browse available feature flags of iroh-1.4.1" class="pure-menu-link">
                    <span class="fa fa-solid fa-flag " aria-hidden="true"></span>
                    <span class="title">Feature flags</span>
                </a>
            </li>
        
    
    <li id="crate-warnings" class="pure-menu-item hidden" data-url="/-/partial/crate-warnings/iroh/"></li>
</ul><div class="spacer"></div>
                
                <div id="abnormalities" class="hidden"></div>

                <ul class="pure-menu-list">
                    <li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="docs.rs">docs.rs</a>
                        <ul class="pure-menu-children aligned-icons"><li class="pure-menu-item"><a class="pure-menu-link" href="/about"><span class="fa fa-solid fa-circle-info " aria-hidden="true"></span> About docs.rs</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/badges"><span class="fa fa-brands fa-fonticons " aria-hidden="true"></span> Badges</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/builds"><span class="fa fa-solid fa-gears " aria-hidden="true"></span> Builds</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/metadata"><span class="fa fa-solid fa-table " aria-hidden="true"></span> Metadata</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/redirections"><span class="fa fa-solid fa-road " aria-hidden="true"></span> Shorthand URLs</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/download"><span class="fa fa-solid fa-download " aria-hidden="true"></span> Download</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/rustdoc-json"><span class="fa fa-solid fa-file-code " aria-hidden="true"></span> Rustdoc JSON</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/releases/queue"><span class="fa fa-solid fa-gears " aria-hidden="true"></span> Build queue</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs" target="_blank"><span class="fa fa-solid fa-shield-halved " aria-hidden="true"></span> Privacy policy</a></li>
                        </ul>
                    </li>
                </ul>
                <ul class="pure-menu-list"><li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="Rust">Rust</a>
                        <ul class="pure-menu-children">
                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://www.rust-lang.org/" target="_blank">Rust website</a></li>
                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/book/" target="_blank">The Book</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/std/" target="_blank">Standard Library API Reference</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/rust-by-example/" target="_blank">Rust by Example</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/cargo/guide/" target="_blank">The Cargo Guide</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/nightly/clippy" target="_blank">Clippy Documentation</a></li>
                        </ul>
                    </li>
                </ul>
                
                <div id="search-input-nav">
                    <label for="nav-search">
                        <span class="fa fa-solid fa-magnifying-glass " aria-hidden="true"></span>
                    </label>

                    
                    
                    <input id="nav-search" name="query" type="text" aria-label="Find crate by search query" tabindex="-1"
                        placeholder="Find crate"
                        >
                </div>
            </form>
        </div>
    </div>
</div>


<div id="alerts" class="hidden"></div><div class="rustdoc mod crate container-rustdoc" id="rustdoc_body_wrapper" tabindex="-1"><script async src="/-/static/v/4ddc5de0b7e3fdcc/menu.js"></script>
<script async src="/-/static/v/4ddc5de0b7e3fdcc/index.js"></script>

<iframe src="/-/storage-change-detection.html" width="0" height="0" style="display: none"></iframe><a class="skip-main-content" href="#main-content">Skip to main content</a><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><rustdoc-topbar><h2><a href="#">Crate iroh</a></h2></rustdoc-topbar><nav class="sidebar"><div class="sidebar-crate"><h2><a href="../iroh/index.html">iroh</a><span class="version">1.4.1</span></h2></div><div class="sidebar-elems"><ul class="block"><li><a id="all-types" href="all.html">All Items</a></li></ul><section id="rustdoc-toc"><h3><a href="#">Sections</a></h3><ul class="block top-toc"><li><a href="#connection-establishment" title="Connection Establishment">Connection Establishment</a></li><li><a href="#encryption" title="Encryption">Encryption</a></li><li><a href="#relay-servers" title="Relay Servers">Relay Servers</a></li><li><a href="#connections-and-streams" title="Connections and Streams">Connections and Streams</a><ul><li><a href="#address-lookup" title="Address Lookup">Address Lookup</a></li></ul></li><li><a href="#examples" title="Examples">Examples</a></li></ul><h3><a href="#reexports">Crate Items</a></h3><ul class="block"><li><a href="#reexports" title="Re-exports">Re-exports</a></li><li><a href="#modules" title="Modules">Modules</a></li><li><a href="#structs" title="Structs">Structs</a></li><li><a href="#enums" title="Enums">Enums</a></li><li><a href="#constants" title="Constants">Constants</a></li><li><a href="#traits" title="Traits">Traits</a></li><li><a href="#types" title="Type Aliases">Type Aliases</a></li></ul></section><div id="rustdoc-modnav"></div></div></nav><div class="sidebar-resizer" title="Drag to resize sidebar"></div><main><div class="width-limiter"><section id="main-content" class="content" tabindex="-1"><div class="main-heading"><h1>Crate <span>iroh</span>&nbsp;<button id="copy-path" title="Copy item path to clipboard">Copy item path</button></h1><rustdoc-toolbar></rustdoc-toolbar><span class="sub-heading"><a class="src" href="../src/iroh/lib.rs.html#1-306">Source</a> </span></div><details class="toggle top-doc" open><summary class="hideme"><span>Expand description</span></summary><div class="docblock"><p>Peer-to-peer QUIC connections.</p>
<p>iroh is a library to establish direct connectivity between peers. It exposes an
interface to <a href="https://quicwg.org">QUIC</a> connections and streams to the user, while implementing direct
connectivity using <a href="https://en.wikipedia.org/wiki/Hole_punching_(networking)">hole punching</a> complemented by relay servers under the hood.</p>
<p>An iroh endpoint is created and controlled by the <a href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint"><code>Endpoint</code></a>, e.g. connecting to
another endpoint:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">let </span>addr: EndpointAddr = <span class="macro">todo!</span>();
<span class="kw">let </span>ep = Endpoint::bind(presets::N0).<span class="kw">await</span><span class="question-mark">?</span>;
<span class="kw">let </span>conn = ep.connect(addr, <span class="string">b"my-alpn"</span>).<span class="kw">await</span><span class="question-mark">?</span>;
<span class="kw">let </span><span class="kw-2">mut </span>send_stream = conn.open_uni().<span class="kw">await</span>.std_context(<span class="string">"unable to open uni"</span>)<span class="question-mark">?</span>;
send_stream
    .write_all(<span class="string">b"msg"</span>)
    .<span class="kw">await
    </span>.std_context(<span class="string">"unable to write all"</span>)<span class="question-mark">?</span>;</code></pre></div>
<p>The other endpoint can accept incoming connections using the <a href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint"><code>Endpoint</code></a> as well:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">let </span>ep = Endpoint::builder(presets::N0)
    .alpns(<span class="macro">vec!</span>[<span class="string">b"my-alpn"</span>.to_vec()])
    .bind()
    .<span class="kw">await</span><span class="question-mark">?</span>;
<span class="kw">let </span>conn = ep
    .accept()
    .<span class="kw">await
    </span>.context(<span class="string">"accept error"</span>)<span class="question-mark">?
    </span>.<span class="kw">await
    </span>.std_context(<span class="string">"connecting error"</span>)<span class="question-mark">?</span>;
<span class="kw">let </span><span class="kw-2">mut </span>recv_stream = conn.accept_uni().<span class="kw">await</span>.std_context(<span class="string">"unable to open uni"</span>)<span class="question-mark">?</span>;
<span class="kw">let </span><span class="kw-2">mut </span>buf = [<span class="number">0u8</span>; <span class="number">3</span>];
recv_stream
    .read_exact(<span class="kw-2">&amp;mut </span>buf)
    .<span class="kw">await
    </span>.std_context(<span class="string">"unable to read"</span>)<span class="question-mark">?</span>;</code></pre></div>
<p>Of course you can also use <a href="endpoint/struct.Connection.html#method.open_bi" title="method iroh::endpoint::Connection::open_bi">bi-directional streams</a> or any other features from QUIC.</p>
<p>For more elaborate examples, see <a href="#examples">below</a> or the examples directory in
the source repository.</p>
<h2 id="connection-establishment"><a class="doc-anchor" href="#connection-establishment">§</a>Connection Establishment</h2>
<p>An iroh connection between two iroh endpoints is usually established with the help
of a Relay server.  When creating the <a href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint"><code>Endpoint</code></a> it connects to the closest Relay
server and designates this as the <em>home relay</em>.  When other endpoints want to connect they
first establish connection via this home relay.  As soon as connection between the two
endpoints is established they will attempt to create a direct connection, using <a href="https://en.wikipedia.org/wiki/Hole_punching_(networking)">hole
punching</a> if needed.  Once the direct connection is established the relay server is no
longer involved in the connection.</p>
<p>If one of the iroh endpoints can be reached directly, connectivity can also be
established without involving a Relay server.  This is done by using the endpoint’s
listening addresses in the connection establishment instead of the <a href="struct.RelayUrl.html" title="struct iroh::RelayUrl"><code>RelayUrl</code></a> which
is used to identify a Relay server.  Of course it is also possible to use both a
<a href="struct.RelayUrl.html" title="struct iroh::RelayUrl"><code>RelayUrl</code></a> and direct addresses at the same time to connect.</p>
<h2 id="encryption"><a class="doc-anchor" href="#encryption">§</a>Encryption</h2>
<p>The connection is encrypted using TLS, like standard QUIC connections.  Unlike standard
QUIC there is no client, server or server TLS key and certificate chain.  Instead each iroh endpoint has a
unique <a href="struct.SecretKey.html" title="struct iroh::SecretKey"><code>SecretKey</code></a> used to authenticate and encrypt the connection.  When an iroh
endpoint connects, it uses the corresponding <a href="struct.PublicKey.html" title="struct iroh::PublicKey"><code>PublicKey</code></a> to ensure the connection is only
established with the intended peer.</p>
<p>Since the <a href="struct.PublicKey.html" title="struct iroh::PublicKey"><code>PublicKey</code></a> is also used to identify the iroh endpoint it is also known as
the <a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a>.  As encryption is an integral part of TLS as used in QUIC this
<a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a> is always a required parameter to establish a connection.</p>
<p>When accepting connections the peer’s <a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a> is authenticated.  However it is up to
the application to decide if a particular peer is allowed to connect or not.</p>
<h2 id="relay-servers"><a class="doc-anchor" href="#relay-servers">§</a>Relay Servers</h2>
<p>Relay servers exist to ensure all iroh endpoints are always reachable.  They accept
<strong>encrypted</strong> traffic for iroh endpoints which are connected to them, forwarding it to
the correct destination based on the <a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a> only.  Since endpoints only send encrypted
traffic, the Relay servers can not decode any traffic for other iroh endpoints and only
forward it.</p>
<p>The connections to the Relay server are initiated as normal HTTP 1.1 connections using
TLS.  Once connected the transport is upgraded to a plain TCP connection using a custom
protocol.  All further data is then sent using this custom relaying protocol.  Usually
soon after the connection is established via the Relay it will migrate to a direct
connection.  However if this is not possible the connection will keep flowing over the
relay server as a fallback.</p>
<p>Additionally to providing reliable connectivity between iroh endpoints, Relay servers
provide some functions to assist in <a href="https://en.wikipedia.org/wiki/Hole_punching_(networking)">hole punching</a>.  They have various services to help
endpoints understand their own network situation.  This includes offering a <a href="https://www.ietf.org/archive/id/draft-ietf-quic-address-discovery-00.html">QAD</a> server,
but also a few HTTP extra endpoints as well as responding to ICMP echo requests.</p>
<p>By default the <a href="https://n0.computer">number 0</a> relay servers are used, see <a href="endpoint/enum.RelayMode.html#variant.Default" title="variant iroh::endpoint::RelayMode::Default"><code>RelayMode::Default</code></a>.</p>
<h2 id="connections-and-streams"><a class="doc-anchor" href="#connections-and-streams">§</a>Connections and Streams</h2>
<p>An iroh endpoint is managed using the <a href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint"><code>Endpoint</code></a> and this is used to create or accept
connections to other endpoints.  To establish a connection to an iroh endpoint you need to
know three pieces of information:</p>
<ul>
<li>The <a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a> of the peer to connect to.</li>
<li>Some addressing information:
<ul>
<li>Usually the <a href="struct.RelayUrl.html" title="struct iroh::RelayUrl"><code>RelayUrl</code></a> identifying the Relay server.</li>
<li>Sometimes, or usually additionally, any direct addresses which might be known.</li>
</ul>
</li>
<li>The QUIC/TLS Application-Layer Protocol Negotiation, or <a href="https://en.wikipedia.org/wiki/Application-Layer_Protocol_Negotiation">ALPN</a>, name to use.</li>
</ul>
<p>The ALPN is used by both sides to agree on which application-specific protocol will be
used over the resulting QUIC connection.  These can be protocols like <code>h3</code> used for
<a href="https://en.wikipedia.org/wiki/HTTP/3">HTTP/3</a>, but more commonly will be a custom identifier for the application.</p>
<p>Once connected the API exposes QUIC streams.  These are very cheap to create so can be
created at any time and can be used to create very many short-lived stream as well as
long-lived streams.  There are two stream types to choose from:</p>
<ul>
<li>
<p><strong>Uni-directional</strong> which only allows the peer which initiated the stream to send
data.</p>
</li>
<li>
<p><strong>Bi-directional</strong> which allows both peers to send and receive data.  However, the
initiator of this stream has to send data before the peer will be aware of this
stream.</p>
</li>
</ul>
<p>Additionally to being extremely light-weight, streams can be interleaved and will not
block each other.  Allowing many streams to co-exist, regardless of how long they last.</p>
<div class="warning">
<p>To keep streams cheap, they are lazily created on the network: only once a sender starts
sending data on the stream will the receiver become aware of a stream.  This means only
calling <a href="endpoint/struct.Connection.html#method.open_bi" title="method iroh::endpoint::Connection::open_bi"><code>Connection::open_bi</code></a> is not sufficient for the corresponding call to
<a href="endpoint/struct.Connection.html#method.accept_bi" title="method iroh::endpoint::Connection::accept_bi"><code>Connection::accept_bi</code></a> to return.  The sender <strong>must</strong> send data on the stream before
the receiver’s <a href="endpoint/struct.Connection.html#method.accept_bi" title="method iroh::endpoint::Connection::accept_bi"><code>Connection::accept_bi</code></a> call will return.</p>
</div>
<h3 id="address-lookup"><a class="doc-anchor" href="#address-lookup">§</a>Address Lookup</h3>
<p>The need to know the <a href="struct.RelayUrl.html" title="struct iroh::RelayUrl"><code>RelayUrl</code></a> <em>or</em> some direct addresses in addition to the
<a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a> to connect to an iroh endpoint can be an obstacle.  To address this, the
<a href="endpoint/struct.Builder.html" title="struct iroh::endpoint::Builder"><code>endpoint::Builder</code></a> allows you to configure an <a href="endpoint/struct.Builder.html#method.address_lookup" title="method iroh::endpoint::Builder::address_lookup"><code>address_lookup</code></a> service.</p>
<p>The <a href="address_lookup/dns/struct.DnsAddressLookup.html" title="struct iroh::address_lookup::dns::DnsAddressLookup"><code>address_lookup::DnsAddressLookup</code></a> service is an address lookup service which will publish the <a href="struct.RelayUrl.html" title="struct iroh::RelayUrl"><code>RelayUrl</code></a>
and direct addresses to a service publishing those as DNS records.  To connect it looks
up the <a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a> in the DNS system to find the addressing details.  This enables
connecting using only the <a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a> which is often more convenient and resilient.</p>
<p>See <a href="address_lookup/index.html" title="mod iroh::address_lookup">the Address Lookup module</a> for more details.</p>
<h2 id="examples"><a class="doc-anchor" href="#examples">§</a>Examples</h2>
<p>The central struct is the <a href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint"><code>Endpoint</code></a>, which allows you to connect to other endpoints:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>iroh::{Endpoint, EndpointAddr, endpoint::presets};
<span class="kw">use </span>n0_error::{<span class="prelude-ty">Result</span>, StackResultExt, StdResultExt};

<span class="kw">async fn </span>connect(addr: EndpointAddr) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
    <span class="comment">// The Endpoint is the central object that manages an iroh node.
    </span><span class="kw">let </span>ep = Endpoint::bind(presets::N0).<span class="kw">await</span><span class="question-mark">?</span>;

    <span class="comment">// Establish a QUIC connection, open a bi-directional stream, exchange messages.
    </span><span class="kw">let </span>conn = ep.connect(addr, <span class="string">b"hello-world"</span>).<span class="kw">await</span><span class="question-mark">?</span>;
    <span class="kw">let </span>(<span class="kw-2">mut </span>send_stream, <span class="kw-2">mut </span>recv_stream) = conn.open_bi().<span class="kw">await</span>.std_context(<span class="string">"open bi"</span>)<span class="question-mark">?</span>;
    send_stream.write_all(<span class="string">b"hello"</span>).<span class="kw">await</span>.std_context(<span class="string">"write"</span>)<span class="question-mark">?</span>;
    send_stream.finish().std_context(<span class="string">"finish"</span>)<span class="question-mark">?</span>;
    <span class="comment">// `read_to_end` waits until the peer finishes or closes its send side.
    </span><span class="kw">let </span>_msg = recv_stream.read_to_end(<span class="number">10</span>).<span class="kw">await</span>.std_context(<span class="string">"read"</span>)<span class="question-mark">?</span>;

    <span class="comment">// Gracefully close the connection and endpoint.
    </span>conn.close(<span class="number">1u8</span>.into(), <span class="string">b"done"</span>);
    ep.close().<span class="kw">await</span>;
    <span class="macro">println!</span>(<span class="string">"Client closed"</span>);
    <span class="prelude-val">Ok</span>(())
}</code></pre></div>
<p>Every <a href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint"><code>Endpoint</code></a> can also accept connections:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>iroh::{Endpoint, EndpointAddr, endpoint::presets};
<span class="kw">use </span>n0_error::{<span class="prelude-ty">Result</span>, StackResultExt, StdResultExt};
<span class="kw">use </span>n0_future::StreamExt;

<span class="kw">async fn </span>accept() -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
    <span class="comment">// To accept connections at least one ALPN must be configured.
    </span><span class="kw">let </span>ep = Endpoint::builder(presets::N0)
        .alpns(<span class="macro">vec!</span>[<span class="string">b"hello-world"</span>.to_vec()])
        .bind()
        .<span class="kw">await</span><span class="question-mark">?</span>;

    <span class="comment">// Accept a QUIC connection, accept a bi-directional stream, exchange messages.
    </span><span class="kw">let </span>conn = ep
        .accept()
        .<span class="kw">await
        </span>.context(<span class="string">"no incoming connection"</span>)<span class="question-mark">?
        </span>.<span class="kw">await
        </span>.context(<span class="string">"accept conn"</span>)<span class="question-mark">?</span>;
    <span class="kw">let </span>(<span class="kw-2">mut </span>send_stream, <span class="kw-2">mut </span>recv_stream) =
        conn.accept_bi().<span class="kw">await</span>.std_context(<span class="string">"accept stream"</span>)<span class="question-mark">?</span>;
    <span class="comment">// `read_to_end` terminates once the client calls `finish` on its send stream.
    </span><span class="kw">let </span>_msg = recv_stream.read_to_end(<span class="number">10</span>).<span class="kw">await</span>.std_context(<span class="string">"read"</span>)<span class="question-mark">?</span>;
    send_stream.write_all(<span class="string">b"world"</span>).<span class="kw">await</span>.std_context(<span class="string">"write"</span>)<span class="question-mark">?</span>;
    send_stream.finish().std_context(<span class="string">"finish"</span>)<span class="question-mark">?</span>;

    <span class="comment">// Wait for the client to close the connection and gracefully close the endpoint.
    </span>conn.closed().<span class="kw">await</span>;
    ep.close().<span class="kw">await</span>;
    <span class="prelude-val">Ok</span>(())
}</code></pre></div>
<p>Please see the examples directory for more nuanced examples.</p>
</div></details><h2 id="reexports" class="section-header">Re-exports<a href="#reexports" class="anchor">§</a></h2><dl class="item-table reexports"><dt id="reexport.Endpoint"><code>pub use endpoint::<a class="struct" href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint">Endpoint</a>;</code></dt><dt id="reexport.RelayMode"><code>pub use endpoint::<a class="enum" href="endpoint/enum.RelayMode.html" title="enum iroh::endpoint::RelayMode">RelayMode</a>;</code></dt></dl><h2 id="modules" class="section-header">Modules<a href="#modules" class="anchor">§</a></h2><dl class="item-table"><dt><a class="mod" href="address_lookup/index.html" title="mod iroh::address_lookup">address_<wbr>lookup</a></dt><dd>Lookup the address of an Endpoint ID.</dd><dt><a class="mod" href="defaults/index.html" title="mod iroh::defaults">defaults</a></dt><dd>Default values used in <a href="index.html" title="mod iroh"><code>iroh</code></a></dd><dt><a class="mod" href="dns/index.html" title="mod iroh::dns">dns</a><wbr><span class="stab portability" title="Available on non-`wasm_browser` only">Non-<code>wasm_browser</code></span></dt><dd>Configurable DNS resolver.</dd><dt><a class="mod" href="endpoint/index.html" title="mod iroh::endpoint">endpoint</a></dt><dd>The <a href="endpoint/struct.Endpoint.html" title="struct iroh::endpoint::Endpoint"><code>Endpoint</code></a> allows establishing connections to other iroh endpoints.</dd><dt><a class="mod" href="endpoint_info/index.html" title="mod iroh::endpoint_info">endpoint_<wbr>info</a></dt><dd>Support for handling DNS resource records for dialing by <a href="type.EndpointId.html" title="type iroh::EndpointId"><code>EndpointId</code></a>.</dd><dt><a class="mod" href="metrics/index.html" title="mod iroh::metrics">metrics</a></dt><dd>Co-locating all of the iroh metrics structs</dd><dt><a class="mod" href="protocol/index.html" title="mod iroh::protocol">protocol</a></dt><dd>Tools for spawning an accept loop that routes incoming requests to the right protocol.</dd><dt><a class="mod" href="test_utils/index.html" title="mod iroh::test_utils">test_<wbr>utils</a><wbr><span class="stab portability" title="Available on crate features `test-utils` only"><code>test-utils</code></span></dt><dd>Internal utilities to support testing.</dd><dt><a class="mod" href="tls/index.html" title="mod iroh::tls">tls</a></dt><dd>TLS configuration for iroh.</dd><dt><a class="mod" href="unstable_net_report/index.html" title="mod iroh::unstable_net_report">unstable_<wbr>net_<wbr>report</a><wbr><span class="stab portability" title="Available on crate feature `unstable-net-report` only"><code>unstable-net-report</code></span></dt><dd>Exports of net report types reachable via <a href="endpoint/struct.Endpoint.html#method.net_report" title="method iroh::endpoint::Endpoint::net_report"><code>crate::endpoint::Endpoint::net_report</code></a>.</dd></dl><h2 id="structs" class="section-header">Structs<a href="#structs" class="anchor">§</a></h2><dl class="item-table"><dt><a class="struct" href="struct.EndpointAddr.html" title="struct iroh::EndpointAddr">Endpoint<wbr>Addr</a></dt><dd>Network-level addressing information for an iroh endpoint.</dd><dt><a class="struct" href="struct.NetReportConfig.html" title="struct iroh::NetReportConfig">NetReport<wbr>Config</a></dt><dd>Configuration for the net report component.</dd><dt><a class="struct" href="struct.PublicKey.html" title="struct iroh::PublicKey">Public<wbr>Key</a></dt><dd>A public key.</dd><dt><a class="struct" href="struct.RelayConfig.html" title="struct iroh::RelayConfig">Relay<wbr>Config</a></dt><dd>Information on a specific relay server.</dd><dt><a class="struct" href="struct.RelayMap.html" title="struct iroh::RelayMap">Relay<wbr>Map</a></dt><dd>List of relay server configurations to be used in an iroh endpoint.</dd><dt><a class="struct" href="struct.RelayUrl.html" title="struct iroh::RelayUrl">Relay<wbr>Url</a></dt><dd>A URL identifying a relay server.</dd><dt><a class="struct" href="struct.RelayUrlParseError.html" title="struct iroh::RelayUrlParseError">Relay<wbr>UrlParse<wbr>Error</a></dt><dd>Can occur when parsing a string into a <a href="struct.RelayUrl.html" title="struct iroh::RelayUrl"><code>RelayUrl</code></a>.</dd><dt><a class="struct" href="struct.SecretKey.html" title="struct iroh::SecretKey">Secret<wbr>Key</a></dt><dd>A secret key.</dd><dt><a class="struct" href="struct.Signature.html" title="struct iroh::Signature">Signature</a></dt><dd>Ed25519 signature.</dd><dt><a class="struct" href="struct.SignatureError.html" title="struct iroh::SignatureError">Signature<wbr>Error</a></dt><dd>Verification of a signature failed.</dd></dl><h2 id="enums" class="section-header">Enums<a href="#enums" class="anchor">§</a></h2><dl class="item-table"><dt><a class="enum" href="enum.KeyParsingError.html" title="enum iroh::KeyParsingError">KeyParsing<wbr>Error</a></dt><dd>Error returned when parsing a <a href="struct.PublicKey.html" title="struct iroh::PublicKey"><code>PublicKey</code></a> or a <a href="struct.SecretKey.html" title="struct iroh::SecretKey"><code>SecretKey</code></a>.</dd><dt><a class="enum" href="enum.TransportAddr.html" title="enum iroh::TransportAddr">Transport<wbr>Addr</a></dt><dd>Available address types.</dd></dl><h2 id="constants" class="section-header">Constants<a href="#constants" class="anchor">§</a></h2><dl class="item-table"><dt><a class="constant" href="constant.NET_REPORT_TIMEOUT.html" title="constant iroh::NET_REPORT_TIMEOUT">NET_<wbr>REPORT_<wbr>TIMEOUT</a></dt><dd>The maximum amount of time, in seconds, the net_report will spend gathering a single report.</dd></dl><h2 id="traits" class="section-header">Traits<a href="#traits" class="anchor">§</a></h2><dl class="item-table"><dt><a class="trait" href="trait.Watcher.html" title="trait iroh::Watcher">Watcher</a></dt><dd>A handle to a value that’s represented by one or more underlying <a href="https://docs.rs/n0-watcher/1.0.0/x86_64-unknown-linux-gnu/n0_watcher/struct.Watchable.html" title="struct n0_watcher::Watchable"><code>Watchable</code></a>s.</dd></dl><h2 id="types" class="section-header">Type Aliases<a href="#types" class="anchor">§</a></h2><dl class="item-table"><dt><a class="type" href="type.EndpointId.html" title="type iroh::EndpointId">Endpoint<wbr>Id</a></dt><dd>The identifier for an endpoint in the (iroh) network.</dd></dl><script type="text/json" id="notable-traits-data">{"&[u8]":"<h3>Notable traits for <code>&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]</div>","<Arc<S> as Service<Request>>::Future":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a R</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;W&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a W</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a>,</div></div>","AcceptBi<'_>":"<h3>Notable traits for <code><a class=\"struct\" href=\"endpoint/struct.AcceptBi.html\" title=\"struct iroh::endpoint::AcceptBi\">AcceptBi</a>&lt;'_&gt;</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"endpoint/struct.AcceptBi.html\" title=\"struct iroh::endpoint::AcceptBi\">AcceptBi</a>&lt;'_&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;(<a class=\"struct\" href=\"endpoint/struct.SendStream.html\" title=\"struct iroh::endpoint::SendStream\">SendStream</a>, <a class=\"struct\" href=\"endpoint/struct.RecvStream.html\" title=\"struct iroh::endpoint::RecvStream\">RecvStream</a>), <a class=\"enum\" href=\"endpoint/enum.ConnectionError.html\" title=\"enum iroh::endpoint::ConnectionError\">ConnectionError</a>&gt;;</div>","AcceptUni<'_>":"<h3>Notable traits for <code><a class=\"struct\" href=\"endpoint/struct.AcceptUni.html\" title=\"struct iroh::endpoint::AcceptUni\">AcceptUni</a>&lt;'_&gt;</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"endpoint/struct.AcceptUni.html\" title=\"struct iroh::endpoint::AcceptUni\">AcceptUni</a>&lt;'_&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"endpoint/struct.RecvStream.html\" title=\"struct iroh::endpoint::RecvStream\">RecvStream</a>, <a class=\"enum\" href=\"endpoint/enum.ConnectionError.html\" title=\"enum iroh::endpoint::ConnectionError\">ConnectionError</a>&gt;;</div>","Arc<B>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a R</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;W&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a W</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a>,</div></div>","Arc<MaybeUninit<T>, A>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a R</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;W&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a W</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a>,</div></div>","Arc<MaybeUninit<T>>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a R</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;W&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a W</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a>,</div></div>","Arc<T, A>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a R</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;W&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a W</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a>,</div></div>","Arc<T>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a R</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;W&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a W</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a>,</div></div>","Arc<U, A>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a R</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/rcs/arc/struct.Arc.html\" title=\"struct alloc::rcs::arc::Arc\">Arc</a>&lt;W&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> + IoHandle + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a W</a>: for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a>,</div></div>","ArrayChunks<Self, N>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/array_chunks/struct.ArrayChunks.html\" title=\"struct core::iter::adapters::array_chunks::ArrayChunks\">ArrayChunks</a>&lt;I, N&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/array_chunks/struct.ArrayChunks.html\" title=\"struct core::iter::adapters::array_chunks::ArrayChunks\">ArrayChunks</a>&lt;I, N&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = [&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">N</a>];</div>","Bytes<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/io/util/struct.Bytes.html\" title=\"struct alloc::io::util::Bytes\">Bytes</a>&lt;R&gt;</code></h3><pre><code><div class=\"where\">impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/io/util/struct.Bytes.html\" title=\"struct alloc::io::util::Bytes\">Bytes</a>&lt;R&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/io/error/struct.Error.html\" title=\"struct core::io::error::Error\">Error</a>&gt;;</div>","Chain<'_>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://docs.rs/n0-error/1.0.1/x86_64-unknown-linux-gnu/n0_error/error/struct.Chain.html\" title=\"struct n0_error::error::Chain\">Chain</a>&lt;'a&gt;</code></h3><pre><code><div class=\"where\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://docs.rs/n0-error/1.0.1/x86_64-unknown-linux-gnu/n0_error/error/struct.Chain.html\" title=\"struct n0_error::error::Chain\">Chain</a>&lt;'a&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = <a class=\"enum\" href=\"https://docs.rs/n0-error/1.0.1/x86_64-unknown-linux-gnu/n0_error/error/enum.ErrorRef.html\" title=\"enum n0_error::error::ErrorRef\">ErrorRef</a>&lt;'a&gt;;</div>","Chain<Self, <U as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html\" title=\"struct core::iter::adapters::chain::Chain\">Chain</a>&lt;A, B&gt;</code></h3><pre><code><div class=\"where\">impl&lt;A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html\" title=\"struct core::iter::adapters::chain::Chain\">Chain</a>&lt;A, B&gt;<div class=\"where\">where\n    A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = &lt;A as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;A as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Chain<Self, R>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/io/util/struct.Chain.html\" title=\"struct core::io::util::Chain\">Chain</a>&lt;T, U&gt;</code></h3><pre><code><div class=\"where\">impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/io/util/struct.Chain.html\" title=\"struct core::io::util::Chain\">Chain</a>&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a>,</div></div>","Cloned<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html\" title=\"struct core::iter::adapters::cloned::Cloned\">Cloned</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;'a, I, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html\" title=\"struct core::iter::adapters::cloned::Cloned\">Cloned</a>&lt;I&gt;<div class=\"where\">where\n    T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a T</a>&gt;,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = T;</div>","Copied<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html\" title=\"struct core::iter::adapters::copied::Copied\">Copied</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;'a, I, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html\" title=\"struct core::iter::adapters::copied::Copied\">Copied</a>&lt;I&gt;<div class=\"where\">where\n    T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a T</a>&gt;,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = T;</div>","Cycle<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html\" title=\"struct core::iter::adapters::cycle::Cycle\">Cycle</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html\" title=\"struct core::iter::adapters::cycle::Cycle\">Cycle</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Enumerate<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html\" title=\"struct core::iter::adapters::enumerate::Enumerate\">Enumerate</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html\" title=\"struct core::iter::adapters::enumerate::Enumerate\">Enumerate</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>);</div>","Filter<Self, P>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html\" title=\"struct core::iter::adapters::filter::Filter\">Filter</a>&lt;I, P&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html\" title=\"struct core::iter::adapters::filter::Filter\">Filter</a>&lt;I, P&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","FilterMap<Self, F>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter_map/struct.FilterMap.html\" title=\"struct core::iter::adapters::filter_map::FilterMap\">FilterMap</a>&lt;I, F&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B, I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter_map/struct.FilterMap.html\" title=\"struct core::iter::adapters::filter_map::FilterMap\">FilterMap</a>&lt;I, F&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;B&gt;,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = B;</div>","FlatMap<Self, U, F>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html\" title=\"struct core::iter::adapters::flatten::FlatMap\">FlatMap</a>&lt;I, U, F&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, U, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html\" title=\"struct core::iter::adapters::flatten::FlatMap\">FlatMap</a>&lt;I, U, F&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; U,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;U as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item\" title=\"type core::iter::traits::collect::IntoIterator::Item\">Item</a>;</div>","Flatten<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.Flatten.html\" title=\"struct core::iter::adapters::flatten::Flatten\">Flatten</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.Flatten.html\" title=\"struct core::iter::adapters::flatten::Flatten\">Flatten</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;IntoIter = U, Item = &lt;U as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;U as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Fuse<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html\" title=\"struct core::iter::adapters::fuse::Fuse\">Fuse</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html\" title=\"struct core::iter::adapters::fuse::Fuse\">Fuse</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Inspect<Self, F>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html\" title=\"struct core::iter::adapters::inspect::Inspect\">Inspect</a>&lt;I, F&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html\" title=\"struct core::iter::adapters::inspect::Inspect\">Inspect</a>&lt;I, F&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>),</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Intersperse<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html\" title=\"struct core::iter::adapters::intersperse::Intersperse\">Intersperse</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html\" title=\"struct core::iter::adapters::intersperse::Intersperse\">Intersperse</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","IntersperseWith<Self, G>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html\" title=\"struct core::iter::adapters::intersperse::IntersperseWith\">IntersperseWith</a>&lt;I, G&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, G&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html\" title=\"struct core::iter::adapters::intersperse::IntersperseWith\">IntersperseWith</a>&lt;I, G&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    G: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>() -&gt; &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Lines<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/io/util/struct.Lines.html\" title=\"struct alloc::io::util::Lines\">Lines</a>&lt;B&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/io/util/struct.Lines.html\" title=\"struct alloc::io::util::Lines\">Lines</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/buf_read/trait.BufRead.html\" title=\"trait alloc::io::buf_read::BufRead\">BufRead</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/io/error/struct.Error.html\" title=\"struct core::io::error::Error\">Error</a>&gt;;</div>","Map<Self, F>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html\" title=\"struct core::iter::adapters::map::Map\">Map</a>&lt;I, F&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B, I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html\" title=\"struct core::iter::adapters::map::Map\">Map</a>&lt;I, F&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; B,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = B;</div>","MapWhile<Self, P>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/map_while/struct.MapWhile.html\" title=\"struct core::iter::adapters::map_while::MapWhile\">MapWhile</a>&lt;I, P&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B, I, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/map_while/struct.MapWhile.html\" title=\"struct core::iter::adapters::map_while::MapWhile\">MapWhile</a>&lt;I, P&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;B&gt;,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = B;</div>","MapWindows<Self, F, N>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/map_windows/struct.MapWindows.html\" title=\"struct core::iter::adapters::map_windows::MapWindows\">MapWindows</a>&lt;I, F, N&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, F, R, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/map_windows/struct.MapWindows.html\" title=\"struct core::iter::adapters::map_windows::MapWindows\">MapWindows</a>&lt;I, F, N&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;[&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">N</a>]) -&gt; R,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = R;</div>","OpenBi<'_>":"<h3>Notable traits for <code><a class=\"struct\" href=\"endpoint/struct.OpenBi.html\" title=\"struct iroh::endpoint::OpenBi\">OpenBi</a>&lt;'_&gt;</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"endpoint/struct.OpenBi.html\" title=\"struct iroh::endpoint::OpenBi\">OpenBi</a>&lt;'_&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;(<a class=\"struct\" href=\"endpoint/struct.SendStream.html\" title=\"struct iroh::endpoint::SendStream\">SendStream</a>, <a class=\"struct\" href=\"endpoint/struct.RecvStream.html\" title=\"struct iroh::endpoint::RecvStream\">RecvStream</a>), <a class=\"enum\" href=\"endpoint/enum.ConnectionError.html\" title=\"enum iroh::endpoint::ConnectionError\">ConnectionError</a>&gt;;</div>","OpenUni<'_>":"<h3>Notable traits for <code><a class=\"struct\" href=\"endpoint/struct.OpenUni.html\" title=\"struct iroh::endpoint::OpenUni\">OpenUni</a>&lt;'_&gt;</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"endpoint/struct.OpenUni.html\" title=\"struct iroh::endpoint::OpenUni\">OpenUni</a>&lt;'_&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"endpoint/struct.SendStream.html\" title=\"struct iroh::endpoint::SendStream\">SendStream</a>, <a class=\"enum\" href=\"endpoint/enum.ConnectionError.html\" title=\"enum iroh::endpoint::ConnectionError\">ConnectionError</a>&gt;;</div>","Peekable<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html\" title=\"struct core::iter::adapters::peekable::Peekable\">Peekable</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html\" title=\"struct core::iter::adapters::peekable::Peekable\">Peekable</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","ReadDatagram<'_>":"<h3>Notable traits for <code><a class=\"struct\" href=\"endpoint/struct.ReadDatagram.html\" title=\"struct iroh::endpoint::ReadDatagram\">ReadDatagram</a>&lt;'_&gt;</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"endpoint/struct.ReadDatagram.html\" title=\"struct iroh::endpoint::ReadDatagram\">ReadDatagram</a>&lt;'_&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://docs.rs/bytes/1.12.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html\" title=\"struct bytes::bytes::Bytes\">Bytes</a>, <a class=\"enum\" href=\"endpoint/enum.ConnectionError.html\" title=\"enum iroh::endpoint::ConnectionError\">ConnectionError</a>&gt;;</div>","ReadManyDatagrams<'a, 'b>":"<h3>Notable traits for <code><a class=\"struct\" href=\"endpoint/struct.ReadManyDatagrams.html\" title=\"struct iroh::endpoint::ReadManyDatagrams\">ReadManyDatagrams</a>&lt;'_, '_&gt;</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"endpoint/struct.ReadManyDatagrams.html\" title=\"struct iroh::endpoint::ReadManyDatagrams\">ReadManyDatagrams</a>&lt;'_, '_&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, <a class=\"enum\" href=\"endpoint/enum.ConnectionError.html\" title=\"enum iroh::endpoint::ConnectionError\">ConnectionError</a>&gt;;</div>","Reader<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://docs.rs/bytes/1.12.1/x86_64-unknown-linux-gnu/bytes/buf/reader/struct.Reader.html\" title=\"struct bytes::buf::reader::Reader\">Reader</a>&lt;B&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/read/trait.Read.html\" title=\"trait alloc::io::read::Read\">Read</a> for <a class=\"struct\" href=\"https://docs.rs/bytes/1.12.1/x86_64-unknown-linux-gnu/bytes/buf/reader/struct.Reader.html\" title=\"struct bytes::buf::reader::Reader\">Reader</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://docs.rs/bytes/1.12.1/x86_64-unknown-linux-gnu/bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,</div></div>","Rev<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/rev/struct.Rev.html\" title=\"struct core::iter::adapters::rev::Rev\">Rev</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/rev/struct.Rev.html\" title=\"struct core::iter::adapters::rev::Rev\">Rev</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Scan<Self, St, F>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html\" title=\"struct core::iter::adapters::scan::Scan\">Scan</a>&lt;I, St, F&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B, I, St, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html\" title=\"struct core::iter::adapters::scan::Scan\">Scan</a>&lt;I, St, F&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut St</a>, &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;B&gt;,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = B;</div>","SendDatagram<'_>":"<h3>Notable traits for <code><a class=\"struct\" href=\"endpoint/struct.SendDatagram.html\" title=\"struct iroh::endpoint::SendDatagram\">SendDatagram</a>&lt;'_&gt;</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"endpoint/struct.SendDatagram.html\" title=\"struct iroh::endpoint::SendDatagram\">SendDatagram</a>&lt;'_&gt;</div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"endpoint/enum.SendDatagramError.html\" title=\"enum iroh::endpoint::SendDatagramError\">SendDatagramError</a>&gt;;</div>","Skip<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html\" title=\"struct core::iter::adapters::skip::Skip\">Skip</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html\" title=\"struct core::iter::adapters::skip::Skip\">Skip</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","SkipWhile<Self, P>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip_while/struct.SkipWhile.html\" title=\"struct core::iter::adapters::skip_while::SkipWhile\">SkipWhile</a>&lt;I, P&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip_while/struct.SkipWhile.html\" title=\"struct core::iter::adapters::skip_while::SkipWhile\">SkipWhile</a>&lt;I, P&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Split<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/io/util/struct.Split.html\" title=\"struct alloc::io::util::Split\">Split</a>&lt;B&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/io/util/struct.Split.html\" title=\"struct alloc::io::util::Split\">Split</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/alloc/io/buf_read/trait.BufRead.html\" title=\"trait alloc::io::buf_read::BufRead\">BufRead</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/io/error/struct.Error.html\" title=\"struct core::io::error::Error\">Error</a>&gt;;</div>","StepBy<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/step_by/struct.StepBy.html\" title=\"struct core::iter::adapters::step_by::StepBy\">StepBy</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/step_by/struct.StepBy.html\" title=\"struct core::iter::adapters::step_by::StepBy\">StepBy</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Take<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html\" title=\"struct core::iter::adapters::take::Take\">Take</a>&lt;I&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html\" title=\"struct core::iter::adapters::take::Take\">Take</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","TakeWhile<Self, P>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/take_while/struct.TakeWhile.html\" title=\"struct core::iter::adapters::take_while::TakeWhile\">TakeWhile</a>&lt;I, P&gt;</code></h3><pre><code><div class=\"where\">impl&lt;I, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/take_while/struct.TakeWhile.html\" title=\"struct core::iter::adapters::take_while::TakeWhile\">TakeWhile</a>&lt;I, P&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;&lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = &lt;I as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>;</div>","Writer<Self>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://docs.rs/bytes/1.12.1/x86_64-unknown-linux-gnu/bytes/buf/writer/struct.Writer.html\" title=\"struct bytes::buf::writer::Writer\">Writer</a>&lt;B&gt;</code></h3><pre><code><div class=\"where\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/io/write/trait.Write.html\" title=\"trait core::io::write::Write\">Write</a> for <a class=\"struct\" href=\"https://docs.rs/bytes/1.12.1/x86_64-unknown-linux-gnu/bytes/buf/writer/struct.Writer.html\" title=\"struct bytes::buf::writer::Writer\">Writer</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://docs.rs/bytes/1.12.1/x86_64-unknown-linux-gnu/bytes/buf/buf_mut/trait.BufMut.html\" title=\"trait bytes::buf::buf_mut::BufMut\">BufMut</a>,</div></div>","Zip<Self, <U as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html\" title=\"struct core::iter::adapters::zip::Zip\">Zip</a>&lt;A, B&gt;</code></h3><pre><code><div class=\"where\">impl&lt;A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html\" title=\"struct core::iter::adapters::zip::Zip\">Zip</a>&lt;A, B&gt;<div class=\"where\">where\n    A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,</div></div><div class=\"where\">    type <a href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = (&lt;A as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>, &lt;B as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>);</div>"}</script></section></div></main></div></body></html>