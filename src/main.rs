use std::{error::Error, fs, process::Command};

use rust_i18n::t;

rust_i18n::i18n!("locales");

markup::define! {
    Button(href: &'static str, icon: &'static str, name: String, alt: &'static str) {
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
    fn new(href: &'static str, icon: &'static str, name: String, alt: &'static str) -> Self {
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
    Hobby(icon: &'static str, name: String) {
        div[class="flex flex-row items-center rounded-sm"] {
            span[class=format!("mr-1 w-8 h-8 {icon} print:hidden")]{}
            @name
        }
    }
}

impl Hobby {
    fn new(icon: &'static str, name: String) -> Self {
        Self { icon, name }
    }
}

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
    fn new(date: String, href: String, color: &'static str, name: &'static str) -> Self {
        Self {
            date,
            href,
            color,
            name,
            items: vec![],
        }
    }

    fn item(mut self, item: String) -> Self {
        self.items.push(item);
        self
    }
}

markup::define! {
    Page<'a>(
        lang: &'a str,
        lang_a: &'a str,
        lang_l: String,
        flag: &'a str,
        img_alt: String,
        buttons: Vec<Button>,
        skills_h: String,
        skills: &'a Vec<Skill>,
        hobbies_h: String,
        hobbies: Vec<Hobby>,
        profile_h: String,
        profile: String,
        exp_h: String,
        edu: String,
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

const LOCALES: [&str; 2] = ["en", "da"];

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

    fs::create_dir_all("_site")?;

    for locale in LOCALES {
        let buttons = vec![
            Button::new(
                "mailto:exempts_chill_0c@icloud.com",
                "i-tabler-mail",
                t!("contact", locale = locale).into(),
                "exempts_chill_0c@icloud.com",
            ),
            Button::new(
                "https://github.com/danbluhmhansen",
                "i-simple-icons-github",
                "GitHub".into(),
                "github.com/danbluhmhansen",
            ),
            Button::new(
                "https://linkedin.com/in/dan-hansen-2b555915b",
                "i-simple-icons-linkedin",
                "LinkedIn".into(),
                "linkedin.com/in/dan-hansen-2b555915b",
            ),
        ];

        let hobbies = vec![
            Hobby::new("i-tabler-run", t!("running", locale = locale).into()),
            Hobby::new("i-tabler-device-gamepad-2", "Gaming".into()),
            Hobby::new("i-simple-icons-linux", "Linux".into()),
            Hobby::new(
                "i-tabler-devices-pc",
                t!("pc-building", locale = locale).into(),
            ),
            Hobby::new("i-tabler-keyboard", t!("keyboards", locale = locale).into()),
        ];

        let experiences = vec![
            Experience::new(
                t!("may", locale = locale).into(),
                "https://itinstituttet.dk".into(),
                "a-neutral",
                "IT Instituttet",
            )
            .item(t!("iti-exp-1", locale = locale).into())
            .item(t!("iti-exp-2", locale = locale).into()),
            Experience::new(
                "September 2020".into(),
                t!("mk-site", locale = locale).into(),
                "a-green",
                "MindKey",
            )
            .item(t!("mk-exp", locale = locale).into()),
            Experience::new(
                "December 2020".into(),
                "https://commentor.dk".into(),
                "a-orange",
                "Commentor",
            )
            .item(t!("comm-exp", locale = locale).into()),
            Experience::new(
                t!("jan", locale = locale).into(),
                t!("ezp-site", locale = locale).into(),
                "a-sky",
                "EazyProject",
            )
            .item(t!("ezp-exp-1", locale = locale).into())
            .item(t!("ezp-exp-2", locale = locale).into())
            .item(t!("ezp-exp-3", locale = locale).into()),
        ];

        fs::write(
            match locale {
                "en" => "_site/index.html",
                "da" => "_site/da.html",
                _ => unreachable!(),
            },
            Page {
                lang: locale,
                lang_a: match locale {
                    "en" => "da.html",
                    "da" => "/",
                    _ => unreachable!(),
                },
                lang_l: t!("lang-link", locale = locale).into(),
                flag: match locale {
                    "en" => "i-flag-dk-4x3",
                    "da" => "i-flag-gb-4x3",
                    _ => unreachable!(),
                },
                img_alt: t!("img-alt", locale = locale).into(),
                buttons,
                skills_h: t!("skills", locale = locale).into(),
                skills,
                hobbies_h: t!("hobbies", locale = locale).into(),
                hobbies,
                profile_h: t!("profile", locale = locale).into(),
                profile: t!("description", locale = locale).into(),
                exp_h: t!("experience", locale = locale).into(),
                edu: t!("education", locale = locale).into(),
                experiences,
            }
            .to_string(),
        )?;
    }

    Ok(())
}
