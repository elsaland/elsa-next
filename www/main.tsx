#!/usr/bin/env deno run --import-map=import_map.json --allow-net --allow-read --unstable --watch

/** @jsx h */
import { serve } from "std/http";
import { router } from "rutt";
import { h, html, tw } from "nanossr";
import { css } from "twind/css";

function Home() {
  return (
    <main
      class={tw`max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-12 lg:py-16`}
    >
      <div class={tw`max-w-3xl mx-auto`}>
        <h1 class={tw`text-4xl font-bold`}>Elsa</h1>
        <p class={tw`text-gray-500`}>
          Elsa is a JavaScript runtime.
        </p>

        <div class={tw`mt-8`}>
          <a
            href="#"
            class={tw`inline-flex items-center justify-center px-5 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700`}
          >
            Get started
          </a>

          <a
            href="#"
            class={tw`ml-3 inline-flex items-center justify-center px-5 py-3 border border-transparent text-base font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200`}
          >
            Learn more
          </a>
        </div>
      </div>

      <div class={tw`mt-8`}>
        <div class={tw`max-w-3xl mx-auto`}>
          <div class={tw`bg-white shadow overflow-hidden sm:rounded-lg`}>
            <div class={tw`px-4 py-5 sm:px-6`}>
              <h3
                class={tw`text-lg

leading-6 font-medium text-gray-900`}
              >
                Recent activity
              </h3>
              <p class={tw`mt-1 max-w-2xl text-sm text-gray-500`}>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Ipsa
                libero labore natus.
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Video */}
      <div class={tw`mt-8 max-w-3xl mx-auto`}>
        <video class={tw`w-full`} controls>
          <source
            src="https://media.w3.org/2010/05/sintel/trailer_hd.mp4"
            type="video/mp4"
          />
        </video>
      </div>

      {/* Features list */}
      <div class={tw`mt-8 max-w-3xl mx-auto`}>
        <h2 class={tw`text-2xl font-bold`}>Features</h2>
        <ul class={tw`mt-4 space-y-4`}>
          <li class={tw`flex`}>
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
            <p class={tw`ml-3 text-base text-gray-500`}>
              Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui
              lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat
              fugiat aliqua.
            </p>
          </li>

          <li class={tw`flex`}>
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
            <p class={tw`ml-3 text-base text-gray-500`}>
              Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui
              lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat
              fugiat aliqua.
            </p>
          </li>
        </ul>
      </div>
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
      }`,
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
