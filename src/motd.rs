use color_eyre::Result;

use crate::fs::FsMaxWidth;
use crate::last_updated::get_last_update_time;
use crate::template::{get_template, render as render_template};
use crate::{
    Config, DateTime, MotdContext, MotdContextBuilder, System, drive_usage,
};

pub(crate) fn render(template_name: &str) -> Result<String> {
    let config = Config::load()?;

    let template = get_template(template_name)?;

    let distro = System::platform_name()?;
    let hostname = System::hostname()?;
    let username = System::username()?;

    let now = DateTime::format_date(DateTime::now());
    let uptime = DateTime::new(System::uptime(&config)?, System::boot_time()?);
    let ram = System::memory_usage()?;

    let filesystems = drive_usage(&config)?;
    let fs_max_width =
        FsMaxWidth::from_filesystems(&filesystems, template.headings_in_width);

    let last_update_time = get_last_update_time(&config)?;

    let context = MotdContextBuilder::default()
        .distro(distro)
        .hostname(hostname)
        .username(username)
        .now(now)
        .uptime(uptime)
        .ram(ram)
        .filesystems(filesystems)
        .fs_max_width(fs_max_width)
        .last_updated(last_update_time)
        .build()?;

    render_context(&context, template_name)
}

fn render_context(
    context: &MotdContext,
    template_name: &str,
) -> Result<String> {
    let template = get_template(template_name)?;

    render_template(context, template.body)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fs::Filesystem;
    use crate::{LastUpdated, Ram};

    fn sample_filesystems() -> Vec<Filesystem> {
        vec![Filesystem {
            fs: "/dev/sda1".to_owned(),
            size: "100G".to_owned(),
            used: "40G".to_owned(),
            avail: "60G".to_owned(),
            pct: "40%".to_owned(),
            target: "/".to_owned(),
        }]
    }

    fn sample_context(last_updated: Option<LastUpdated>) -> MotdContext {
        let filesystems = sample_filesystems();
        let fs_max_width = FsMaxWidth::from_filesystems(&filesystems, false);

        MotdContextBuilder::default()
            .distro("Arch Linux".to_owned())
            .hostname("testbox".to_owned())
            .username("stef".to_owned())
            .now("Mon Sep 23 14:08:11 2024".to_owned())
            .uptime(DateTime::new(
                "2d 3h".to_owned(),
                "Sat Sep 21 11:08:11 2024".to_owned(),
            ))
            .last_updated(last_updated)
            .ram(Ram::new(
                "4096".to_owned(),
                "50".to_owned(),
                "8192".to_owned(),
            ))
            .filesystems(filesystems)
            .fs_max_width(fs_max_width)
            .build()
            .unwrap()
    }

    #[test]
    fn test_render_context_json_renders_expected_fields() -> Result<()> {
        let context = sample_context(Some(LastUpdated::new(
            "4d".to_owned(),
            "Thu Sep 19 14:08:11 2024".to_owned(),
            "pacman".to_owned(),
        )));

        let rendered = render_context(&context, "json")?;

        assert!(rendered.contains("\u{1b}[94m\"user@host\":\u{1b}[0m"));
        assert!(
            rendered.contains(
                "\u{1b}[92m\u{1b}[1m\"stef@testbox\"\u{1b}[0m\u{1b}[0m"
            )
        );
        assert!(rendered.contains("\u{1b}[96m\u{1b}[1m\"Arch Linux\""));
        assert!(rendered.contains(
            "\u{1b}[94m\"updated\":\u{1b}[0m [\u{1b}[93m\u{1b}[1m\"4d\""
        ));
        assert!(rendered.contains("\u{1b}[36m\"/dev/sda1\":"));

        Ok(())
    }

    #[test]
    fn test_render_context_md_omits_update_section_when_absent() -> Result<()> {
        let context = sample_context(None);

        let rendered = render_context(&context, "md")?;

        assert!(
            rendered
                .contains("# \u{1b}[96m\u{1b}[1mArch Linux\u{1b}[0m\u{1b}[0m")
        );
        assert!(rendered.contains("## \u{1b}[94mSystem status\u{1b}[0m at "));
        assert!(!rendered.contains("last updated on"));

        Ok(())
    }

    #[test]
    fn test_render_context_toml_renders_union_of_existing_template_data()
    -> Result<()> {
        let context = sample_context(Some(LastUpdated::new(
            "4d".to_owned(),
            "Thu Sep 19 14:08:11 2024".to_owned(),
            "pacman".to_owned(),
        )));

        let rendered = render_context(&context, "toml")?;

        assert!(rendered.contains("\u{1b}[94muser_at_host\u{1b}[0m = "));
        assert!(
            rendered.contains(
                "\u{1b}[92m\u{1b}[1m\"stef@testbox\"\u{1b}[0m\u{1b}[0m"
            )
        );
        assert!(rendered.contains("\u{1b}[94mnow\u{1b}[0m = "));
        assert!(rendered.contains("\u{1b}[94muptime\u{1b}[0m = { "));
        assert!(rendered.contains("\u{1b}[94mdrives\u{1b}[0m = ["));
        assert!(rendered.contains("\u{1b}[90mpct\u{1b}[0m = "));
        assert!(rendered.contains("\u{1b}[94mlast_updated\u{1b}[0m = { "));
        assert!(rendered.contains("\u{1b}[36mapp\u{1b}[0m = "));

        Ok(())
    }
}
