markup::define! {
    Hobby(icon: &'static str, name: String) {
        div[class="flex flex-row items-center rounded-sm"] {
            span[class=format!("mr-1 w-8 h-8 {icon} print:hidden")]{}
            @name
        }
    }
}

impl Hobby {
    pub fn new(icon: &'static str, name: String) -> Self {
        Self { icon, name }
    }
}
