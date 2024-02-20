use maud::{html, Markup};

pub fn skill(
    href: &'static str,
    name: &'static str,
    icon: Option<&'static str>,
    color: Option<&'static str>,
) -> Markup {
    html! {
        a
            href=(href)
            target="_blank"
            class="flex flex-row items-center rounded-sm focus:ring focus:ring-blue-500 focus:outline-none focus:ring-offset-3 focus:dark:ring-offset-stone-900" {
            @if let Some((icon, color)) = icon.zip(color) {
                span class={"mr-1 h-8 w-8 print:hidden " (icon) " " (color)} {}
            }
            (name)
        }
    }
}
