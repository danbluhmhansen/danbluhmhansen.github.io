use maud::{html, Markup};

pub fn button(href: &'static str, icon: &'static str, name: String, alt: &'static str) -> Markup {
    html! {
        a
            href=(href)
            target="_blank"
            class="flex flex-row gap-1 justify-center items-center p-2 bg-gradient-to-r from-cyan-300 to-blue-300 rounded-xl dark:from-cyan-500 dark:to-blue-500 hover:from-cyan-400 hover:to-blue-400 focus:ring focus:outline-none active:from-cyan-500 active:to-blue-500 print:hidden dark:hover:from-cyan-600 dark:hover:to-blue-600 dark:active:from-cyan-700 dark:active:to-blue-700" {
            span class={"w-6 h-6 stroke-2 " (icon)} {}
            (name)
        }
        div class="hidden text-xs print:block" { (alt) }
    }
}
