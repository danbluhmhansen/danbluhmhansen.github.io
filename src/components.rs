use self::{button::Button, experience::Experience, hobby::Hobby, skill::Skill};

pub mod button;
pub mod experience;
pub mod hobby;
pub mod skill;

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
