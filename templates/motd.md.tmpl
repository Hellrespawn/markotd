# {{ distro | bold | color("bright_cyan") }} on {{ hostname | bold }}

The current user is {{ username | bold | color("bright_green") }}.

## {{ "System status" | color("bright_blue") }} at {{ now | dim }}

{{ hostname | color("cyan") }} is up {{ uptime.time_since | bold | color("bright_green") }}, since {{ uptime.date | dim }}.

{{ ram.used | bold | color("bright_green") }} MB of {{ ram.total | color("cyan") }} GB ({{ ram.pct | bold | color("yellow") }}%) RAM is in use.

### {{ "Drive Usage" | color("bright_blue") }}

| {{ "filesystem" | ljust(fs_max_width.fs) | bold | color("bright_white") }} | {{ "size" | rjust(fs_max_width.size) | bold | color("bright_white") }} | {{ "used" | rjust(fs_max_width.used) | bold | color("bright_white") }} | {{ "avail" | rjust(fs_max_width.avail) | bold | color("bright_white") }} | {{ "pct" | rjust(fs_max_width.pct) | bold | color("bright_white") }} | {{ "target" | ljust(fs_max_width.target) | bold | color("bright_white") }} |
| {{ "-" | repeat(fs_max_width.fs) }} | {{ "-" | repeat(fs_max_width.size) }} | {{ "-" | repeat(fs_max_width.used) }} | {{ "-" | repeat(fs_max_width.avail) }} | {{ "-" | repeat(fs_max_width.pct) }} | {{ "-" | repeat(fs_max_width.target) }} |
{%- for drive in filesystems %}
| {{ drive.fs | ljust(fs_max_width.fs) | color("cyan") }} | {{ drive.size | rjust(fs_max_width.size) }} | {{ drive.used | rjust(fs_max_width.used) }} | {{ drive.avail | rjust(fs_max_width.avail) }} | {{ drive.pct | rjust(fs_max_width.pct) | bold | color("yellow") }} | {{ drive.target | ljust(fs_max_width.target) }} |
{%- endfor %}
{% if last_updated %}
## {{ last_updated.app | color("bright_blue") }} last updated on {{ last_updated.date | dim }}

It has been {{ last_updated.time_since | bold | color("bright_yellow") }}.
{% endif %}
