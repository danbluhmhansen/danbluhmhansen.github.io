use maud::{html, Markup};

pub fn hobby(icon: &'static str, name: String) -> Markup {
    html! {
        div class="flex flex-row items-center rounded-sm" {
            span class={"mr-1 w-8 h-8 print:hidden " (icon)} {}
            (name)
        }
    }
}
