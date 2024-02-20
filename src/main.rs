use std::{error::Error, fs, process::Command};

use components::{button::button, experience::experience, hobby::hobby, page, skill::skill};
use rust_i18n::t;

mod components;

rust_i18n::i18n!("locales");

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
        skill(
            "https://learn.microsoft.com/en-us/aspnet/core",
            "ASP.NET",
            Some("i-simple-icons-dotnet"),
            Some("text-[#512bd4]"),
        ),
        skill(
            "https://learn.microsoft.com/en-us/azure",
            "Azure",
            Some("i-simple-icons-microsoftazure"),
            Some("text-[#0078d4]"),
        ),
        skill(
            "https://learn.microsoft.com/en-us/azure/devops",
            "Azure DevOps",
            Some("i-simple-icons-azuredevops"),
            Some("text-[#0078d4]"),
        ),
        skill(
            "https://docker.com",
            "Docker",
            Some("i-simple-icons-docker"),
            Some("text-[#2496ed]"),
        ),
        skill(
            "https://learn.microsoft.com/en-us/ef",
            "Entity Framework",
            Some("i-devicon-plain-dot-net"),
            Some("text-[#1384c8]"),
        ),
        skill(
            "https://graphql.org",
            "GraphQL",
            Some("i-simple-icons-graphql"),
            Some("text-[#e10098]"),
        ),
        skill("https://oauth.net", "OAuth 2.0", None, None),
        skill("https://www.odata.org", "OData", None, None),
        skill(
            "https://openapis.org",
            "OpenAPI",
            Some("i-simple-icons-openapiinitiative"),
            Some("text-[#6ba539]"),
        ),
        skill(
            "https://openid.net/connect",
            "OpenID Connect",
            Some("i-simple-icons-openid"),
            Some("text-[#f78c40]"),
        ),
        skill(
            "https://postgresql.org",
            "PostgreSQL",
            Some("i-simple-icons-postgresql"),
            Some("text-[#4169e1]"),
        ),
        skill(
            "https://react.dev",
            "React",
            Some("i-simple-icons-react"),
            Some("text-[#61dafb]"),
        ),
        skill(
            "https://rust-lang.org",
            "Rust",
            Some("i-simple-icons-rust"),
            Some("dark:text-gray-500"),
        ),
        skill(
            "https://learn.microsoft.com/en-us/sql",
            "SQL Server",
            Some("i-simple-icons-microsoftsqlserver"),
            Some("text-[#cc2927]"),
        ),
        skill(
            "https://terraform.io",
            "Terraform",
            Some("i-simple-icons-terraform"),
            Some("text-[#7b42bc]"),
        ),
    ];

    fs::create_dir_all("_site")?;

    for locale in LOCALES {
        let buttons = vec![
            button(
                "mailto:exempts_chill_0c@icloud.com",
                "i-tabler-mail",
                t!("contact", locale = locale).into(),
                "exempts_chill_0c@icloud.com",
            ),
            button(
                "https://github.com/danbluhmhansen",
                "i-simple-icons-github",
                "GitHub".into(),
                "github.com/danbluhmhansen",
            ),
            button(
                "https://linkedin.com/in/dan-hansen-2b555915b",
                "i-simple-icons-linkedin",
                "LinkedIn".into(),
                "linkedin.com/in/dan-hansen-2b555915b",
            ),
        ];

        let hobbies = vec![
            hobby("i-tabler-run", t!("running", locale = locale).into()),
            hobby("i-tabler-device-gamepad-2", "Gaming".into()),
            hobby("i-simple-icons-linux", "Linux".into()),
            hobby(
                "i-tabler-devices-pc",
                t!("pc-building", locale = locale).into(),
            ),
            hobby("i-tabler-keyboard", t!("keyboards", locale = locale).into()),
        ];

        let experiences = vec![
            experience(
                t!("may", locale = locale).into(),
                "https://itinstituttet.dk".into(),
                "a-neutral",
                "IT Instituttet",
                vec![
                    t!("iti-exp-1", locale = locale).into(),
                    t!("iti-exp-2", locale = locale).into(),
                ],
            ),
            experience(
                "September 2020".into(),
                t!("mk-site", locale = locale).into(),
                "a-green",
                "MindKey",
                vec![t!("mk-exp", locale = locale).into()],
            ),
            experience(
                "December 2020".into(),
                "https://commentor.dk".into(),
                "a-orange",
                "Commentor",
                vec![t!("comm-exp", locale = locale).into()],
            ),
            experience(
                t!("jan", locale = locale).into(),
                t!("ezp-site", locale = locale).into(),
                "a-sky",
                "EazyProject",
                vec![
                    t!("ezp-exp-1", locale = locale).into(),
                    t!("ezp-exp-2", locale = locale).into(),
                    t!("ezp-exp-3", locale = locale).into(),
                ],
            ),
        ];

        fs::write(
            match locale {
                "en" => "_site/index.html",
                "da" => "_site/da.html",
                _ => unreachable!(),
            },
            page(
                locale,
                match locale {
                    "en" => "da.html",
                    "da" => "/",
                    _ => unreachable!(),
                },
                t!("lang-link", locale = locale).into(),
                match locale {
                    "en" => "i-flag-dk-4x3",
                    "da" => "i-flag-gb-4x3",
                    _ => unreachable!(),
                },
                t!("img-alt", locale = locale).into(),
                buttons,
                t!("skills", locale = locale).into(),
                skills,
                t!("hobbies", locale = locale).into(),
                hobbies,
                t!("profile", locale = locale).into(),
                t!("description", locale = locale).into(),
                t!("experience", locale = locale).into(),
                t!("education", locale = locale).into(),
                experiences,
            )
            .into_string(),
        )?;
    }

    Ok(())
}
