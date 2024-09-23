# {{distro}} on {{hostname}}

The current user is {{username}}.

## System status at Mon Sep 23 14:08:11 2024

{{hostname}} is up {{uptime.time}}, since {{uptime.date}}.

{{ram.used}} MB of {{ram.total}} GB ({{ram.pct}}%) RAM is in use.

### Drive Usage

| filesystem     |  size | used | avail | pct | mount       |
| -------------- | ----- | ---- | ----- | --- | ----------- |
{%- for drive in drives %}
| {{drive.fs}} |  {{drive.size}} | {{drive.used}} |  {{drive.avail}} | {{drive.pct}}% | {{drive.target}} |
{%- endfor %}
