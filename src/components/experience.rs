markup::define! {
    Experience(
        date: String,
        href: String,
        color: &'static str,
        name: &'static str,
        items: Vec<String>,
    ) {
        li {
            p[class="text-sm text-gray-500 print:text-xs"] { @date }
            a[href=href,target="_blank",class=color] {
                h3[class="inline text-lg print:text-base"] { @name }
            }
            ul[class="list-disc list-inside print:text-xs"] { @for item in items { li { @item } } }
        }
    }
}

impl Experience {
    pub fn new(date: String, href: String, color: &'static str, name: &'static str) -> Self {
        Self {
            date,
            href,
            color,
            name,
            items: vec![],
        }
    }

    pub fn item(mut self, item: String) -> Self {
        self.items.push(item);
        self
    }
}
