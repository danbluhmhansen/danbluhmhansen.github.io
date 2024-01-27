use std::{error::Error, fs, process::Command};

use components::{button::Button, experience::Experience, hobby::Hobby, skill::Skill, Page};
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
