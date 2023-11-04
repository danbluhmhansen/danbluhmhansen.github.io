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

fn main() -> Result<(), Box<dyn Error>> {
    Command::new("sh").arg("-c").arg("bun install").output()?;
    Command::new("sh").arg("-c").arg("bun run bld").output()?;

    let buttons = vec![
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

    let skills = vec![
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

    let hobbies = vec![
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

    let experiences = vec![
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
            href: "https://mindkey.com/",
            color: "a-green",
            name: "MindKey",
            items: vec![
                "Developed a number of Azure Functions for various features, generating error reports,
                synchronising data between services.",
            ],
        },
        Experience {
            date: "December 2020",
            href: "https://commentor.dk/",
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

    let page = markup::new! {
        @markup::doctype()
        html[lang="en",class="dark:text-white dark:bg-slate-900"] {
            head {
                meta[charset="utf-8"];
                meta[name="viewport",content="width=device-width,initial-scale=1"];
                title { "Dan Bluhm Hansen - CV" }
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
                    }
                    div[class="grid gap-8 py-8 sm:grid-cols-3 print:grid-cols-3 print:gap-4"] {
                        aside[class="flex flex-col col-span-3 gap-4 sm:col-span-1 print:col-span-1"] {
                            img[
                                width="640",
                                height="640",
                                src="https://danbluhmhansen.github.io/cv/profile_lg.avif",
                                srcset="
                                    https://danbluhmhansen.github.io/cv/profile_sm.avif 160w,
                                    https://danbluhmhansen.github.io/cv/profile_md.avif 320w,
                                    https://danbluhmhansen.github.io/cv/profile_lg.avif 640w
                                ",
                                alt="Bald white male, with short full beard, dressed in a navy blazer and gray shirt."
                            ];
                            section[class="flex flex-col gap-2"] { @for button in &buttons { @button } }
                            section {
                                h2[class="text-lg print:text-base"] { "Skills" }
                                div[class="flex flex-wrap gap-y-2 gap-x-4 print:gap-x-2 print:gap-y-0"] {
                                    @for skill in &skills { @skill }
                                }
                            }
                            section {
                                h2[class="text-lg print:text-base"] { "Hobbies" }
                                div[class="flex flex-wrap gap-y-2 gap-x-4 print:gap-x-2 print:gap-y-0"] {
                                    @for hobby in &hobbies { @hobby }
                                }
                            }
                        }
                        main[class="flex col-span-3 justify-center sm:col-span-2 print:col-span-2"] {
                            section[class="flex flex-col gap-4 print:gap-2"] {
                                h2[class="text-3xl text-indigo-600 dark:text-indigo-300 print:text-xl"] { "Profile" }
                                p[class="print:text-xs"] {
                                    "I am a passionate software developer who strives to build secure, stable, and fast software solutions. I spend much of my time researching the latest frameworks and tools so that I can keep up with the ever-changing software world. I love to learn and to share what I have learned with my colleagues, so we can all enjoy developing software together. What I like most about software development is building tools and foundations for others so they can focus on implementation and business logic. As a person, I am modest, calm, and always willing to help others."
                                }
                                h2[class="text-3xl text-indigo-600 dark:text-indigo-300 print:text-xl"] { "Experience" }
                                ol[class="space-y-2"] {
                                    li {
                                        p[class="text-sm text-gray-500 print:text-xs"] { "September 2015" }
                                        h3[class="inline text-lg print:text-base"] {
                                            "Academy Profession (AP) degree in Computer Science - Zealand Institute of Business and Technology"
                                        }
                                    }
                                    @for experience in &experiences { @experience }
                                }
                            }
                        }
                    }
                    footer[class="flex flex-row gap-2 justify-end print:hidden"] {
                        a[
                            href="https://github.com/danbluhmhansen/cv",
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
    };

    fs::create_dir_all("_site")?;
    fs::write("_site/index.html", page.to_string())?;

    Ok(())
}
