use std::{error::Error, fs, process::Command};

markup::define! {
    Button(href: &'static str, icon: &'static str, name: &'static str, alt: &'static str) {
        a[href=href,target="_blank",class="btn"] {
            span[class=format!("w-6 h-6 stroke-2 {icon}")]{}
            @name
        }
        div[class="hidden text-xs print:block"] { @alt }
    }
}

markup::define! {
    Skill(href: &'static str, name: &'static str, icon: Option<&'static str>, color: Option<&'static str>) {
        a[href=href,target="_blank",class="skill"] {
            @if let Some((icon, color)) = icon.zip(*color) {
                span[class=format!("{icon} mr-1 h-8 w-8 {color} print:hidden")]{}
            }
            @name
        }
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
                link[
                  rel="icon",
                  type="image/svg+xml",
                  href="data:image/svg+xml,%3C%3Fxml version='1.0' encoding='UTF-8' standalone='no'%3F%3E%3Csvg width='385pt' height='385pt' viewBox='0 0 385 385' version='1.1' id='svg30' xmlns:xlink='https://w3.org/1999/xlink' xmlns='https://w3.org/2000/svg' xmlns:svg='https://w3.org/2000/svg'%3E%3Cdefs id='defs16'%3E%3Cg id='g11'%3E%3Cg id='glyph-0-0'%3E%3Cpath d='M 7.921875 0 L 7.921875 -184.921875 L 145.296875 -184.921875 L 145.296875 0 Z M 76.609375 -109.109375 L 114.921875 -163.78125 L 38.3125 -163.78125 Z M 31.703125 -44.90625 L 64.984375 -92.453125 L 31.703125 -140.28125 Z M 121.515625 -44.90625 L 121.515625 -140.28125 L 88.234375 -92.453125 Z M 38.3125 -21.140625 L 114.921875 -21.140625 L 76.609375 -76.078125 Z M 38.3125 -21.140625 ' id='path2' /%3E%3C/g%3E%3Cg id='glyph-0-1'%3E%3Cpath d='M 39.625 -21.140625 L 128.125 -21.140625 L 128.125 -163.78125 L 39.625 -163.78125 Z M 15.84375 0 L 15.84375 -184.921875 L 130.765625 -184.921875 L 151.90625 -163.78125 L 151.90625 -21.140625 L 130.765625 0 Z M 15.84375 0 ' id='path5' /%3E%3C/g%3E%3Cg id='glyph-0-2'%3E%3Cpath d='M 15.84375 0 L 15.84375 -184.921875 L 39.625 -184.921875 L 39.625 -105.671875 L 137.375 -105.671875 L 137.375 -184.921875 L 161.140625 -184.921875 L 161.140625 0 L 137.375 0 L 137.375 -84.53125 L 39.625 -84.53125 L 39.625 0 Z M 15.84375 0 ' id='path8' /%3E%3C/g%3E%3C/g%3E%3CclipPath id='clip-0'%3E%3Cpath clip-rule='nonzero' d='M 0 0 L 385 0 L 385 385 L 0 385 Z M 0 0 ' id='path13' /%3E%3C/clipPath%3E%3C/defs%3E%3Cg clip-path='url(%23clip-0)' id='g20'%3E%3Cpath fill-rule='nonzero' fill='%231e1e2e' fill-opacity='1' d='m 0.375,0.375 h 384.25 v 384.25 H 0.375 Z m 0,0' id='path18' /%3E%3C/g%3E%3Cg fill='%23f5e0dc' fill-opacity='1' id='g24'%3E%3Cuse xlink:href='%23glyph-0-1' x='19.747196' y='284.43466' id='use22' /%3E%3C/g%3E%3Cg fill='%23f5e0dc' fill-opacity='1' id='g28'%3E%3Cuse xlink:href='%23glyph-0-2' x='187.49611' y='284.43466' id='use26' /%3E%3C/g%3E%3C/svg%3E%0A",
                ];
                link[
                    rel="stylesheet",
                    type="text/css",
                    href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind-compat.min.css"
                ];
                link[rel="stylesheet",type="text/css",href="site.css"];
            }
            body[class="container min-h-screen min-w-full p-4 dark:bg-gradient-to-br dark:from-slate-900 dark:to-stone-900 sm:p-8"] {
                div[class="mx-auto max-w-screen-lg"] {
                    header[class="flex flex-row justify-between items-center"] {
                        h1[class="text-5xl print:text-3xl"] { "Dan Bluhm Hansen" }
                        a[href=lang_a,"aria-label"=lang_l,class="print:hidden"] {
                            div[class=format!("w-8 h-8 {flag}")]{}
                        }
                    }
                    div[class="sm:grid gap-8 py-8 sm:grid-cols-2 md:grid-cols-3 print:grid-cols-3 print:gap-4"] {
                        aside[class="flex flex-col items-center gap-4 sm:col-span-1 print:col-span-1"] {
                            img[
                                width="320",
                                height="320",
                                src="https://danbluhmhansen.github.io/cv/profile_md.avif",
                                srcset="
                                    https://danbluhmhansen.github.io/cv/profile_sm.avif 160w,
                                    https://danbluhmhansen.github.io/cv/profile_md.avif 320w,
                                    https://danbluhmhansen.github.io/cv/profile_lg.avif 640w
                                ",
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
                        main[class="flex justify-center sm:col-span-1 md:col-span-2 print:col-span-2"] {
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
    Command::new("sh").arg("-c").arg("bun run bld").output()?;

    let buttons_en = vec![
        Button {
            href: "mailto:exempts_chill_0c@icloud.com",
            icon: "i-tabler-mail",
            name: "Contact",
            alt: "exempts_chill_0c@icloud.com",
        },
        Button {
            href: "https://github.com/danbluhmhansen",
            icon: "i-simple-icons-github",
            name: "GitHub",
            alt: "github.com/danbluhmhansen",
        },
        Button {
            href: "https://linkedin.com/in/dan-hansen-2b555915b",
            icon: "i-simple-icons-linkedin",
            name: "LinkedIn",
            alt: "linkedin.com/in/dan-hansen-2b555915b",
        },
    ];

    let buttons_da = vec![
        Button {
            href: "mailto:exempts_chill_0c@icloud.com",
            icon: "i-tabler-mail",
            name: "Kontakt",
            alt: "exempts_chill_0c@icloud.com",
        },
        Button {
            href: "https://github.com/danbluhmhansen",
            icon: "i-simple-icons-github",
            name: "GitHub",
            alt: "github.com/danbluhmhansen",
        },
        Button {
            href: "https://linkedin.com/in/dan-hansen-2b555915b",
            icon: "i-simple-icons-linkedin",
            name: "LinkedIn",
            alt: "linkedin.com/in/dan-hansen-2b555915b",
        },
    ];

    let skills = &vec![
        Skill {
            href: "https://learn.microsoft.com/en-us/aspnet/core",
            name: "ASP.NET",
            icon: Some("i-simple-icons-dotnet"),
            color: Some("text-[#512bd4]"),
        },
        Skill {
            href: "https://learn.microsoft.com/en-us/azure",
            name: "Azure",
            icon: Some("i-simple-icons-microsoftazure"),
            color: Some("text-[#0078d4]"),
        },
        Skill {
            href: "https://learn.microsoft.com/en-us/azure/devops",
            name: "Azure DevOps",
            icon: Some("i-simple-icons-azuredevops"),
            color: Some("text-[#0078d4]"),
        },
        Skill {
            href: "https://docker.com",
            name: "Docker",
            icon: Some("i-simple-icons-docker"),
            color: Some("text-[#2496ed]"),
        },
        Skill {
            href: "https://learn.microsoft.com/en-us/ef",
            name: "Entity Framework",
            icon: Some("i-devicon-plain-dot-net"),
            color: Some("text-[#1384c8]"),
        },
        Skill {
            href: "https://graphql.org",
            name: "GraphQL",
            icon: Some("i-simple-icons-graphql"),
            color: Some("text-[#e10098]"),
        },
        Skill {
            href: "https://oauth.net",
            name: "OAuth 2.0",
            icon: None,
            color: None,
        },
        Skill {
            href: "https://www.odata.org",
            name: "OData",
            icon: None,
            color: None,
        },
        Skill {
            href: "https://openapis.org",
            name: "OpenAPI",
            icon: Some("i-simple-icons-openapiinitiative"),
            color: Some("text-[#6ba539]"),
        },
        Skill {
            href: "https://openid.net/connect",
            name: "OpenID Connect",
            icon: Some("i-simple-icons-openid"),
            color: Some("text-[#f78c40]"),
        },
        Skill {
            href: "https://postgresql.org",
            name: "PostgreSQL",
            icon: Some("i-simple-icons-postgresql"),
            color: Some("text-[#4169e1]"),
        },
        Skill {
            href: "https://react.dev",
            name: "React",
            icon: Some("i-simple-icons-react"),
            color: Some("text-[#61dafb]"),
        },
        Skill {
            href: "https://rust-lang.org",
            name: "Rust",
            icon: Some("i-simple-icons-rust"),
            color: Some("dark:text-gray-500"),
        },
        Skill {
            href: "https://learn.microsoft.com/en-us/sql",
            name: "SQL Server",
            icon: Some("i-simple-icons-microsoftsqlserver"),
            color: Some("text-[#cc2927]"),
        },
        Skill {
            href: "https://terraform.io",
            name: "Terraform",
            icon: Some("i-simple-icons-terraform"),
            color: Some("text-[#7b42bc]"),
        },
    ];

    let hobbies_en = vec![
        Hobby {
            icon: "i-tabler-run",
            name: "Running",
        },
        Hobby {
            icon: "i-tabler-device-gamepad-2",
            name: "Gaming",
        },
        Hobby {
            icon: "i-simple-icons-linux",
            name: "Linux",
        },
        Hobby {
            icon: "i-tabler-devices-pc",
            name: "PC building",
        },
        Hobby {
            icon: "i-tabler-keyboard",
            name: "Mechanical keyboards",
        },
    ];

    let hobbies_da = vec![
        Hobby {
            icon: "i-tabler-run",
            name: "Løb",
        },
        Hobby {
            icon: "i-tabler-device-gamepad-2",
            name: "Gaming",
        },
        Hobby {
            icon: "i-simple-icons-linux",
            name: "Linux",
        },
        Hobby {
            icon: "i-tabler-devices-pc",
            name: "PC-bygning",
        },
        Hobby {
            icon: "i-tabler-keyboard",
            name: "Mekaniske tastaturer",
        },
    ];

    let experiences_en = vec![
        Experience {
            date: "May 2017",
            href: "https://itinstituttet.dk",
            color: "a-neutral",
            name: "IT Instituttet",
            items: vec![
                "Developed the backend for a cloud-based property management system (PMS),
                that targets the hospitality industry.",
                "Worked with many different aspects of software development, date and time management,
                authorisation and authentication, APIs, and integration with hardware systems such as Nets payment
                terminals and Axis network door controllers.",
            ],
        },
        Experience {
            date: "September 2020",
            href: "https://mindkey.com",
            color: "a-green",
            name: "MindKey",
            items: vec![
                "Developed a number of Azure Functions for various features, generating error reports,
                synchronising data between services.",
            ],
        },
        Experience {
            date: "December 2020",
            href: "https://commentor.dk",
            color: "a-orange",
            name: "Commentor",
            items: vec![
                "Part of a large platform lift for PensionDanmark across multiple projects, CRM, login, and more.",
            ],
        },
        Experience {
            date: "January 2022",
            href: "https://eazyproject.net/en",
            color: "a-sky",
            name: "EazyProject",
            items: vec![
                "Lead on a platform lift to migrate an ASP.NET Web Forms app to a modern platform,
                and bring it to the cloud.",
                "Built a multi-tenant server that can manage multiple databases with differing schemas.
                Supporting OAuth 2.0, OpenID Connect, and a GraphQL API.",
                "Later got the role of Tech Lead and responsibility of the company's tech stack,
                software infrastructure, DevOps, and developer training.",
            ],
        },
    ];

    let experiences_da = vec![
        Experience {
            date: "Maj 2017",
            href: "https://itinstituttet.dk",
            color: "a-neutral",
            name: "IT Instituttet",
            items: vec![
                "Udviklede backend til et cloud-baseret ejendomsadministration system (PMS), der er målrettet mod hotel-
                og restaurationsbranchen.",
                "Arbejdede med mange forskellige aspekter af softwareudvikling, dato- og tidsstyring, authorization og
                authentication API'er og integration med hardwaresystemer som Nets betalingsterminaler og Axis
                netværksdørcontrollere.",
            ],
        },
        Experience {
            date: "September 2020",
            href: "https://mindkey.dk",
            color: "a-green",
            name: "MindKey",
            items: vec![
                "Udviklet en række Azure Functions til forskellige funktioner, generering af fejlrapporter,
                synkronisering af data mellem tjenester.",
            ],
        },
        Experience {
            date: "December 2020",
            href: "https://commentor.dk",
            color: "a-orange",
            name: "Commentor",
            items: vec![
                "Del af et stort platformslift for PensionDanmark på tværs flere projekter, CRM, login og mere.",
            ],
        },
        Experience {
            date: "Januar 2022",
            href: "https://eazyproject.net",
            color: "a-sky",
            name: "EazyProject",
            items: vec![
                "Lead på et platformslift for at opgradere en ASP.NET Web Forms-app til en moderne platform, og bringe
                den i skyen.",
                "Bygget en multi-tenant server, der kan administrere flere databaser med forskellige skemaer.
                Understøtter OAuth 2.0, OpenID Connect, og med et GraphQL API.",
                "Senere fik rollen som Tech Lead og ansvar for virksomhedens tech stack, softwareinfrastruktur, DevOps
                og uddannelse af udviklere.",
            ],
        },
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
