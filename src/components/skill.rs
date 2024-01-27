markup::define! {
    Skill(href: &'static str, name: &'static str, icon: Option<&'static str>, color: Option<&'static str>) {
        a[
            href=href,
            target="_blank",
            class="flex flex-row items-center rounded-sm focus:ring focus:ring-blue-500 focus:outline-none focus:ring-offset-3 focus:dark:ring-offset-stone-900",
        ] {
            @if let Some((icon, color)) = icon.zip(*color) {
                span[class=format!("{icon} mr-1 h-8 w-8 {color} print:hidden")]{}
            }
            @name
        }
    }
}

impl Skill {
    pub fn new(href: &'static str, name: &'static str) -> Self {
        Self {
            href,
            name,
            icon: None,
            color: None,
        }
    }

    pub fn icon(mut self, icon: &'static str, color: &'static str) -> Self {
        self.icon = Some(icon);
        self.color = Some(color);
        self
    }
}
