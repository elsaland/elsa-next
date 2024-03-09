#!/usr/bin/env deno run --import-map=import_map.json --allow-net --allow-read --unstable --watch

/** @jsx h */
import { serve } from "std/http";
import { router } from "rutt";
import { h, html, tw } from "nanossr";
import { css } from "twind/css";

function Button({ children, ...props }) {
  return (
    <a
      class={tw`inline-flex items-center justify-center px-5 py-3 border border-transparent text-base font-medium rounded-md ${props.class}`}
    >
      {children}
    </a>
  );
}

function GitHub({ ...props }) {
  return (
    <a
      target="_blank"
      href="https://github.com/elsaland/elsa"
      {...props}
    >
      <img
        src="https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
        alt="GitHub icon"
        width="32"
        height="32"
      />
    </a>
  );
}

function IconGreenCheck() {
  return (
    <div class={tw`flex-shrink-0`}>
      <svg
        class={tw`h-6 w-6 text-green-500`}
        x-description="Heroicon name: solid/check"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        aria-hidden="true"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M5 13l4 4L19 7"
        />
      </svg>
    </div>
  );
}

const features = [
  [
    "",
    "Highly configurable. Tons of granular options to tweak every aspect of the runtime.",
  ],
  [
    "",
    "Embeddable. Rust crate and C API for embedding anywhere.",
  ],
  [
    "",
    "Engine agnostic. Choose your JS engine! V8, JavaScript Core, QuickJS, etc.",
  ],
  [
    "",
    "Speed. You choose your engine, Elsa provides optimized bindings for your engine.",
  ],
  [
    "",
    "Quick compilation. Elsa is designed to be compiled quickly, with minimal dependencies.",
  ],
];

function Home() {
  return (
    <main
      class={tw`max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-12 lg:py-16`}
    >
      <div class={tw`max-w-3xl mx-auto`}>
        <h1 class={tw`text-4xl font-bold`}>Elsa</h1>
        <p class={tw`text-gray-500 dark:text-gray-200`}>
          Low level JavaScript runtime, close to the metal.
        </p>

        <div class={tw`mt-8`}>
          <img
            src="https://v8.dev/_img/v8.svg"
            alt="V8"
            width="38"
            height="38"
            class={tw`inline-block`}
          />
          <img
            src="https://docs.webkit.org/assets/WebKit.svg"
            alt="JSC"
            width="38"
            height="38"
            class={tw`inline-block ml-2`}
          />
          <img
            src="https://raw.githubusercontent.com/facebook/hermes/main/website/static/img/logo.svg"
            alt="Hermes"
            width="38"
            height="38"
            class={tw`inline-block ml-2`}
          />
          <img
            src="https://upload.wikimedia.org/wikipedia/commons/thumb/8/89/Spidermonkey-logo-2021.svg/120px-Spidermonkey-logo-2021.svg.png"
            alt="SpiderMonkey"
            width="38"
            height="38"
            class={tw`inline-block ml-2`}
          />
          <img
            src="https://avatars.githubusercontent.com/u/149480552?s=200&v=4"
            alt="QuickJS"
            width="38"
            height="38"
            class={tw`inline-block ml-2`}
          />
        </div>

        <GitHub class={tw`absolute top-0 right-0 mt-4 mr-4`} />
      </div>

      <div class={tw`mt-8 max-w-3xl mx-auto`}>
        <div
          class={tw`bg-white overflow-hidden sm:rounded-lg border border-gray-200 dark:border-gray-800`}
        >
          <div class={tw`px-4 py-5 sm:px-6 pre`}>
            <p class={tw`mt-1 max-w-2xl text-sm text-gray-500`}>
              <code>
                {`brew install elsa`}
              </code>
            </p>
          </div>
        </div>
      </div>
      <div
        class={tw`mt-8 max-w-3xl mx-auto`}
      >
        <Button
          class={tw`text-white text-indigo-700 bg-indigo-100`}
          style="backdrop-filter: blur(10px);"
        >
          Coming soon...
        </Button>
      </div>

      {/* Features list */}
      <div class={tw`mt-8 max-w-3xl mx-auto`}>
        <ul class={tw`mt-4 space-y-4`}>
          {features.map(([title, description]) => (
            <li class={tw`flex`}>
              <IconGreenCheck />
              <p class={tw`ml-3 text-base text-gray-500 dark:text-gray-200`}>
                {description}
              </p>
            </li>
          ))}
        </ul>
      </div>

      {
        /* Small build walkthrough
      <script
        src="https://asciinema.org/a/545330.js"
        data-theme="asciinema"
        data-size="big"
        data-autoplay="true"
        id="asciicast-545330"
        async
      >
      </script> */
      }
    </main>
  );
}

const options = {
  preflight: (preflight: string) =>
    css`
      ${preflight}
      @import url('https://fonts.googleapis.com/css2?family=Pacifico&display=swap');
      h1 {
        font-family: 'Pacifico';
      }
      @media (prefers-color-scheme: dark) {
        body {
          background-color: #121212;
          color: #ffffff;
        }
        .pre {
          background-color: #292929;
          color: #ffffff;
        }
        code {
          color: #ffffff;
        }
      }
      `,
};

function memoize<T>(fn: () => T): () => T {
  let cache: T | undefined;
  return () => {
    if (cache) return cache;
    cache = fn();
    return cache;
  };
}

const start = typeof Deno.serve !== "undefined" ? Deno.serve : serve;

const home = memoize(() => html(() => <Home />, { tw: options }));

await serve(
  router({
    "/": () =>
      new Response(home(), { headers: { "content-type": "text/html" } }),
  }) as Deno.ServeHandler,
);
