use std::{error::Error, fs, process::Command};

markup::define! {
    Button(href: &'static str, icon: &'static str, name: &'static str, alt: &'static str) {
        a[
            href=href,
            target="_blank",
            class="flex flex-row gap-1 justify-center items-center p-2 bg-gradient-to-r from-cyan-300 to-blue-300 rounded-xl dark:from-cyan-500 dark:to-blue-500 hover:from-cyan-400 hover:to-blue-400 focus:ring focus:outline-none active:from-cyan-500 active:to-blue-500 print:hidden dark:hover:from-cyan-600 dark:hover:to-blue-600 dark:active:from-cyan-700 dark:active:to-blue-700",
        ] {
            span[class=format!("w-6 h-6 stroke-2 {icon}")]{}
            @name
        }
        div[class="hidden text-xs print:block"] { @alt }
    }
}

impl Button {
    fn new(href: &'static str, icon: &'static str, name: &'static str, alt: &'static str) -> Self {
        Self {
            href,
            icon,
            name,
            alt,
        }
    }
}

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
    fn new(href: &'static str, name: &'static str) -> Self {
        Self {
            href,
            name,
            icon: None,
            color: None,
        }
    }

    fn icon(mut self, icon: &'static str, color: &'static str) -> Self {
        self.icon = Some(icon);
        self.color = Some(color);
        self
    }
}

markup::define! {
    Hobby(icon: &'static str, name: &'static str) {
        div[class="flex flex-row items-center rounded-sm"] {
            span[class=format!("mr-1 w-8 h-8 {icon} print:hidden")]{}
            @name
        }
    }
}

impl Hobby {
    fn new(icon: &'static str, name: &'static str) -> Self {
        Self { icon, name }
    }
}

markup::define! {
    Experience(
        date: &'static str,
        href: &'static str,
        color: &'static str,
        name: &'static str,
        items: Vec<&'static str>,
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
    fn new(
        date: &'static str,
        href: &'static str,
        color: &'static str,
        name: &'static str,
    ) -> Self {
        Self {
            date,
            href,
            color,
            name,
            items: vec![],
        }
    }

    fn item(mut self, item: &'static str) -> Self {
        self.items.push(item);
        self
    }
}

markup::define! {
    Page<'a>(
        lang: &'a str,
        lang_a: &'a str,
        lang_l: &'a str,
        flag: &'a str,
        img_alt: &'a str,
        buttons: Vec<Button>,
        skills_h: &'a str,
        skills: &'a Vec<Skill>,
        hobbies_h: &'a str,
        hobbies: Vec<Hobby>,
        profile_h: &'a str,
        profile: &'a str,
        exp_h: &'a str,
        edu: &'a str,
        experiences: Vec<Experience>,
    ) {
        @markup::doctype()
        html[lang=lang,class="dark:text-white dark:bg-slate-900"] {
            head {
                meta[charset="utf-8"];
                meta[name="viewport",content="width=device-width,initial-scale=1"];
                title { "Dan Bluhm Hansen - CV" }
                link[rel="apple-touch-icon",sizes="180x180",href="apple-touch-icon.png"];
                link[rel="icon",type="image/png",sizes="32x32",href="favicon-32x32.png"];
                link[rel="icon",type="image/png",sizes="16x16",href="favicon-16x16.png"];
                link[rel="manifest",href="site.webmanifest"];
                link[
                    rel="stylesheet",
                    type="text/css",
                    href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind-compat.min.css"
                ];
                link[rel="stylesheet",type="text/css",href="site.css"];
            }
            body[
                class="container p-4 min-w-full min-h-screen sm:p-8 dark:bg-gradient-to-br dark:from-slate-900 dark:to-stone-900"
            ] {
                div[class="mx-auto max-w-screen-lg"] {
                    header[class="flex flex-row justify-between items-center"] {
                        h1[class="text-5xl print:text-3xl"] { "Dan Bluhm Hansen" }
                        a[href=lang_a,"aria-label"=lang_l,class="print:hidden"] {
                            div[class=format!("w-8 h-8 {flag}")]{}
                        }
                    }
                    div[class="grid gap-8 py-8 sm:grid-cols-3 print:gap-4"] {
                        aside[class="flex flex-col gap-4 items-center"] {
                            img[
                                width="320",
                                height="320",
                                src="https://danbluhmhansen.github.io/cv/profile_md.avif",
                                srcset="https://danbluhmhansen.github.io/cv/profile_sm.avif 160w,https://danbluhmhansen.github.io/cv/profile_md.avif 320w,https://danbluhmhansen.github.io/cv/profile_lg.avif 640w",
                                alt=img_alt
                            ];
                            section[class="flex flex-col gap-2 w-full"] { @for button in buttons { @button } }
                            section {
                                h2[class="text-lg print:text-base"] { @skills_h }
                                div[class="flex flex-wrap gap-y-2 gap-x-4 print:gap-x-2 print:gap-y-0"] {
                                    @for skill in *skills { @skill }
                                }
                            }
                            section {
                                h2[class="text-lg print:text-base"] { @hobbies_h }
                                div[class="flex flex-wrap gap-y-2 gap-x-4 print:gap-x-2 print:gap-y-0"] {
                                    @for hobby in hobbies { @hobby }
                                }
                            }
                        }
                        main[class="flex justify-center sm:col-span-2"] {
                            section[class="flex flex-col gap-4 print:gap-2"] {
                                h2[class="text-3xl text-indigo-600 dark:text-indigo-300 print:text-xl"] { @profile_h }
                                p[class="print:text-xs"] { @profile }
                                h2[class="text-3xl text-indigo-600 dark:text-indigo-300 print:text-xl"] { @exp_h }
                                ol[class="space-y-2"] {
                                    li {
                                        p[class="text-sm text-gray-500 print:text-xs"] { "September 2015" }
                                        h3[class="inline text-lg print:text-base"] { @edu }
                                    }
                                    @for experience in experiences { @experience }
                                }
                            }
                        }
                    }
                    footer[class="flex flex-row gap-2 justify-end print:hidden"] {
                        a[
                            href="https://github.com/danbluhmhansen/danbluhmhansen.github.io",
                            target="_blank",
                            "aria-label"="Source code",
                            class="btm-i"
                        ] { span[class="w-8 h-8 i-simple-icons-github"]{} }
                        button[onclick="window.print();","aria-label"="Print page",class="btm-i"] {
                            span[class="w-8 h-8 i-tabler-printer"]{}
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Command::new("sh").arg("-c").arg("bun install").output()?;
    Command::new("sh")
        .arg("-c")
        .arg(if cfg!(debug_assertions) {
            "bun run bld"
        } else {
            "bun run min"
        })
        .output()?;

    let buttons_en = vec![
        Button::new(
            "mailto:exempts_chill_0c@icloud.com",
            "i-tabler-mail",
            "Contact",
            "exempts_chill_0c@icloud.com",
        ),
        Button::new(
            "https://github.com/danbluhmhansen",
            "i-simple-icons-github",
            "GitHub",
            "github.com/danbluhmhansen",
        ),
        Button::new(
            "https://linkedin.com/in/dan-hansen-2b555915b",
            "i-simple-icons-linkedin",
            "LinkedIn",
            "linkedin.com/in/dan-hansen-2b555915b",
        ),
    ];

    let buttons_da = vec![
        Button::new(
            "mailto:exempts_chill_0c@icloud.com",
            "i-tabler-mail",
            "Kontakt",
            "exempts_chill_0c@icloud.com",
        ),
        Button::new(
            "https://github.com/danbluhmhansen",
            "i-simple-icons-github",
            "GitHub",
            "github.com/danbluhmhansen",
        ),
        Button::new(
            "https://linkedin.com/in/dan-hansen-2b555915b",
            "i-simple-icons-linkedin",
            "LinkedIn",
            "linkedin.com/in/dan-hansen-2b555915b",
        ),
    ];

    let skills = &vec![
        Skill::new("https://learn.microsoft.com/en-us/aspnet/core", "ASP.NET")
            .icon("i-simple-icons-dotnet", "text-[#512bd4]"),
        Skill::new("https://learn.microsoft.com/en-us/azure", "Azure")
            .icon("i-simple-icons-microsoftazure", "text-[#0078d4]"),
        Skill::new(
            "https://learn.microsoft.com/en-us/azure/devops",
            "Azure DevOps",
        )
        .icon("i-simple-icons-azuredevops", "text-[#0078d4]"),
        Skill::new("https://docker.com", "Docker").icon("i-simple-icons-docker", "text-[#2496ed]"),
        Skill::new("https://learn.microsoft.com/en-us/ef", "Entity Framework")
            .icon("i-devicon-plain-dot-net", "text-[#1384c8]"),
        Skill::new("https://graphql.org", "GraphQL")
            .icon("i-simple-icons-graphql", "text-[#e10098]"),
        Skill::new("https://oauth.net", "OAuth 2.0"),
        Skill::new("https://www.odata.org", "OData"),
        Skill::new("https://openapis.org", "OpenAPI")
            .icon("i-simple-icons-openapiinitiative", "text-[#6ba539]"),
        Skill::new("https://openid.net/connect", "OpenID Connect")
            .icon("i-simple-icons-openid", "text-[#f78c40]"),
        Skill::new("https://postgresql.org", "PostgreSQL")
            .icon("i-simple-icons-postgresql", "text-[#4169e1]"),
        Skill::new("https://react.dev", "React").icon("i-simple-icons-react", "text-[#61dafb]"),
        Skill::new("https://rust-lang.org", "Rust")
            .icon("i-simple-icons-rust", "dark:text-gray-500"),
        Skill::new("https://learn.microsoft.com/en-us/sql", "SQL Server")
            .icon("i-simple-icons-microsoftsqlserver", "text-[#cc2927]"),
        Skill::new("https://terraform.io", "Terraform")
            .icon("i-simple-icons-terraform", "text-[#7b42bc]"),
    ];

    let hobbies_en = vec![
        Hobby::new("i-tabler-run", "Running"),
        Hobby::new("i-tabler-device-gamepad-2", "Gaming"),
        Hobby::new("i-simple-icons-linux", "Linux"),
        Hobby::new("i-tabler-devices-pc", "PC building"),
        Hobby::new("i-tabler-keyboard", "Mechanical keyboards"),
    ];

    let hobbies_da = vec![
        Hobby::new("i-tabler-run", "Løb"),
        Hobby::new("i-tabler-device-gamepad-2", "Gaming"),
        Hobby::new("i-simple-icons-linux", "Linux"),
        Hobby::new("i-tabler-devices-pc", "PC-bygning"),
        Hobby::new("i-tabler-keyboard", "Mekaniske tastaturer"),
    ];

    let experiences_en = vec![
        Experience::new("May 2017", "https://itinstituttet.dk", "a-neutral", "IT Instituttet")
            .item("Developed the backend for a cloud-based property management system (PMS), that targets the hospitality industry.")
            .item("Worked with many different aspects of software development, date and time management, authorisation and authentication, APIs, and integration with hardware systems such as Nets payment terminals and Axis network door controllers."),
        Experience::new("September 2020", "https://mindkey.com", "a-green", "MindKey")
            .item("Developed a number of Azure Functions for various features, generating error reports, synchronising data between services."),
        Experience::new("December 2020", "https://commentor.dk", "a-orange", "Commentor")
            .item("Part of a large platform lift for PensionDanmark across multiple projects, CRM, login, and more."),
        Experience::new("January 2022", "https://eazyproject.net/en", "a-sky", "EazyProject")
            .item("Lead on a platform lift to migrate an ASP.NET Web Forms app to a modern platform, and bring it to the cloud.")
            .item("Built a multi-tenant server that can manage multiple databases with differing schemas. Supporting OAuth 2.0, OpenID Connect, and a GraphQL API.")
            .item("Later got the role of Tech Lead and responsibility of the company's tech stack, software infrastructure, DevOps, and developer training."),
    ];

    let experiences_da = vec![
        Experience::new("Maj 2017", "https://itinstituttet.dk", "a-neutral", "IT Instituttet")
            .item("Udviklede backend til et cloud-baseret ejendomsadministration system (PMS), der er målrettet mod hotel- og restaurationsbranchen.")
            .item("Arbejdede med mange forskellige aspekter af softwareudvikling, dato- og tidsstyring, authorization og authentication API'er og integration med hardwaresystemer som Nets betalingsterminaler og Axis netværksdørcontrollere."),
        Experience::new("September 2020", "https://mindkey.dk", "a-green", "MindKey")
            .item("Udviklet en række Azure Functions til forskellige funktioner, generering af fejlrapporter, synkronisering af data mellem tjenester."),
        Experience::new("December 2020", "https://commentor.dk", "a-orange", "Commentor")
            .item("Del af et stort platformslift for PensionDanmark på tværs flere projekter, CRM, login og mere."),
        Experience::new("Januar 2022", "https://eazyproject.net", "a-sky", "EazyProject")
            .item("Lead på et platformslift for at opgradere en ASP.NET Web Forms-app til en moderne platform, og bringe den i skyen.")
            .item("Bygget en multi-tenant server, der kan administrere flere databaser med forskellige skemaer. Understøtter OAuth 2.0, OpenID Connect, og med et GraphQL API.")
            .item("Senere fik rollen som Tech Lead og ansvar for virksomhedens tech stack, softwareinfrastruktur, DevOps og uddannelse af udviklere."),
    ];

    fs::create_dir_all("_site")?;
    fs::write("_site/index.html", Page {
        lang: "en",
        lang_a: "da.html",
        lang_l: "Gå til engelsk site",
        flag: "i-flag-dk-4x3",
        img_alt: "Bald white male, with short full beard, dressed in a navy blazer and gray shirt.",
        buttons: buttons_en,
        skills_h: "Skills",
        skills,
        hobbies_h: "Hobbies",
        hobbies: hobbies_en,
        profile_h: "Profile",
        profile: "I am a passionate software developer who strives to build secure, stable, and fast software solutions. I spend much of my time researching the latest frameworks and tools so that I can keep up with the ever-changing software world. I love to learn and to share what I have learned with my colleagues, so we can all enjoy developing software together. What I like most about software development is building tools and foundations for others so they can focus on implementation and business logic. As a person, I am modest, calm, and always willing to help others.",
        exp_h: "Experience",
        edu: "Academy Profession (AP) degree in Computer Science - Zealand Institute of Business and Technology",
        experiences: experiences_en,
    }.to_string())?;

    fs::write("_site/da.html", Page {
        lang: "da",
        lang_a: "index.html",
        lang_l: "Go to danish site",
        flag: "i-flag-gb-4x3",
        img_alt: "Skaldet hvid mand, med kort fuldskæg, klædt i en marineblå blazer og grå skjorte.",
        buttons: buttons_da,
        skills_h: "Færdigheder",
        skills,
        hobbies_h: "Hobbyer",
        hobbies: hobbies_da,
        profile_h: "Profil",
        profile: "Jeg er en passioneret softwareudvikler, der stræber efter at bygge sikre, stabile og hurtige softwareløsninger. Jeg bruger meget af min tid på at undersøge de nyeste frameworks og værktøjer, så jeg kan holde trit med den evigt udviklende softwareverden. Jeg elsker at lære og dele det jeg har lært med mine kolleger, så vi alle kan nyde at udvikle software sammen. Det jeg bedst kan lide ved softwareudvikling, er at bygge værktøjer og fundamenter for andre, så de kan fokusere på implementering og forretningslogik. Som person er jeg beskeden, rolig og altid villig til at hjælpe andre.",
        exp_h: "Erfaring",
        edu: " Datamatiker - Erhvervsakademi Sjælland ",
        experiences: experiences_da,
    }.to_string())?;

    Ok(())
}
