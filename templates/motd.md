# {{distro}} on {{hostname}}

The current user is {{username}}.

## System status at Mon Sep 23 14:08:11 2024

{{hostname}} is up {{uptime.time_since}}, since {{uptime.date}}.

{{ram.used}} MB of {{ram.total}} GB ({{ram.pct}}%) RAM is in use.

### Drive Usage

| {{ "filesystem" | ljust(fs_max_width.fs) }} | {{ "size" | rjust(fs_max_width.size) }} | {{ "used" | rjust(fs_max_width.used) }} | {{ "avail" | rjust(fs_max_width.avail) }} | {{ "pct" | rjust(fs_max_width.pct + 1) }} | {{ "target" | ljust(fs_max_width.target) }} |
| {{ "-" | repeat(fs_max_width.fs) }} | {{ "-" | repeat(fs_max_width.size) }} | {{ "-" | repeat(fs_max_width.used) }} | {{ "-" | repeat(fs_max_width.avail) }} | {{ "-" | repeat(fs_max_width.pct + 1) }} | {{ "-" | repeat(fs_max_width.target) }} |
{%- for drive in filesystems %}
| {{drive.fs | ljust(fs_max_width.fs) }} | {{drive.size | rjust(fs_max_width.size) }} | {{drive.used | rjust(fs_max_width.used) }} | {{drive.avail | rjust(fs_max_width.avail) }} | {{drive.pct | rjust(fs_max_width.pct) }}% | {{drive.target | ljust(fs_max_width.target) }} |
{%- endfor %}
{% if last_updated %}
## {{ last_updated.app }} last updated on {{last_updated.date }}

It has been {{ last_updated.time_since }}.
{% endif %}
