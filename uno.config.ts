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
        flag: () => import("@iconify-json/flag/icons.json").then((i) => i.default),
        "simple-icons": () => import("@iconify-json/simple-icons/icons.json").then((i) => i.default),
        tabler: () => import("@iconify-json/tabler/icons.json").then((i) => i.default),
      },
    }),
  ],
  shortcuts: [
    {
      "btm-i": "inline-flex bg-transparent rounded-sm focus:ring focus:ring-blue-500 focus:outline-none focus:ring-offset-3 focus:dark:ring-offset-stone-900",
    },
    [/^a-(.*)$/, ([, c]) => `underline rounded-sm focus:ring focus:ring-blue-500 focus:ring-offset-4 focus:outline-none print:no-underline decoration-2 focus:dark:ring-offset-stone-900 decoration-${c}-300 dark:decoration-${c}-500 dark:hover:decoration-${c}-600 dark:active:decoration-${c}-700 hover:decoration-${c}-400 active:decoration-${c}-500`],
  ],
});
