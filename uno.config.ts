import { defineConfig, presetUno } from "unocss";
import presetIcons from "@unocss/preset-icons";

export default defineConfig({
  cli: {
    entry: {
      outFile: "_site/site.css",
      patterns: ["src/**/*.rs"],
    },
  },
  presets: [
    presetUno({ dark: "media" }),
    presetIcons({
      collections: {
        "devicon-plain": () => import("@iconify-json/devicon-plain/icons.json").then((i) => i.default),
        "simple-icons": () => import("@iconify-json/simple-icons/icons.json").then((i) => i.default),
        tabler: () => import("@iconify-json/tabler/icons.json").then((i) => i.default),
      },
    }),
  ],
  shortcuts: [
    {
      "body": "container p-4 mx-auto min-w-full max-w-screen-lg min-h-screen sm:p-8 dark:bg-gradient-to-br dark:from-slate-900 dark:to-stone-900",
      "btn": "flex flex-row gap-1 justify-center items-center p-2 bg-gradient-to-r from-cyan-300 to-blue-300 rounded-xl dark:from-cyan-500 dark:to-blue-500 hover:from-cyan-400 hover:to-blue-400 focus:ring focus:outline-none active:from-cyan-500 active:to-blue-500 print:hidden dark:hover:from-cyan-600 dark:hover:to-blue-600 dark:active:from-cyan-700 dark:active:to-blue-700",
      "a": "underline rounded-sm focus:ring focus:ring-blue-500 focus:ring-offset-4 focus:outline-none print:no-underline decoration-2 focus:dark:ring-offset-stone-900",
      "skill": "flex flex-row items-center rounded-sm focus:ring focus:ring-blue-500 focus:outline-none focus:ring-offset-3 focus:dark:ring-offset-stone-900",
      "btm-i": "inline-flex bg-transparent rounded-sm focus:ring focus:ring-blue-500 focus:outline-none focus:ring-offset-3 focus:dark:ring-offset-stone-900",
    },
    [/^a-(.*)$/, ([, c]) => `a decoration-${c}-300 dark:decoration-${c}-500 dark:hover:decoration-${c}-600 dark:active:decoration-${c}-700 hover:decoration-${c}-400 active:decoration-${c}-500`],
  ],
});
