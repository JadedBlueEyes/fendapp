# let html_content = (pandoc --from markdown --to html5 --section-divs CHANGELOG.md )
export def parse_changelog_html [html_content: string] {
    let matches = $html_content | parse -r "<section\\s+id=\"[a-z0-9\\-]*\"\\s+class=\"level2\"\\s*>([\\W\\w]*?)</section>" | skip 1 | get capture0

    if ($env | get DRY_RUN -o | default 'manual') == "true" {
        let unreleased_version = $env | get -o NEW_VERSION
        let unreleased_url = if ($unreleased_version | is-not-empty) { $'https://github.com/JadedBlueEyes/fendapp/releases/tag/($unreleased_version)' }
    }

    # Iterate over each match and extract version, date, URL, and description
    let releases = $matches | each { |e|
        let version = $e | parse -r "<h2>\\s*<a[^>]*>(?P<version>.*?)</a>" | get -o version | get -o 0
        let date = $e | parse -r "- (?P<date>[\\d]{4}(-[\\d]{2}){2})" | get -o date | get -o 0 | default (date now | format date "%F")
        let release_url = $e | parse -r "<h2>\\s*<a\\s+href=\"(?P<release_url>[^\"]*)\"" | get -o release_url | get -o 0
        let parsed_desc = ($e | parse -r '(a>|(h2>Unreleased changes))(:\s*(?P<tagline>[\W\w]*?))?(\s*- (?P<date>[\d]{4}(-[\d]{2}){2}))?<\/h2>\s*(?P<body>[\W\w]*)')
        let description = if ($parsed_desc | is-empty) { "" } else {
            $parsed_desc | each { |d|
                let ver_text = if ($version == null) { 'Unreleased changes' } else { $version }
                let tagline_text = if ($d.tagline == null) { '' } else if (($d.tagline | str trim) != "") { $': ($d.tagline)' } else { '' }
                let body_text = $d.body | str replace -r '<p\s*>\s*Full Changelog:([\W\w]*?)([\W\w]*?)<\/p>' ''
                $"<p>($ver_text)($tagline_text)</p>($body_text)"
            } | get -o 0
        }

        # Print the extracted information
        {version: ($version | default "Unreleased") date: $date release_url: ($release_url) description: $description}
    }

    if ($env | get DRY_RUN -o | default 'manual') == "false" {
        return ($releases | where {|e| ($e.version) != "Unreleased"})
    } else {
        return $releases
    }
}
