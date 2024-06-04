# let html_content = (pandoc --from markdown --to html5 --section-divs CHANGELOG.md )
export def parse_changelog_html [html_content: string] {
    let matches = $html_content | parse -r "<section\\s+id=\"[a-z0-9\\-]*\"\\s+class=\"level2\"\\s*>([\\W\\w]*?)</section>" | get capture0

    if ($env | get DRY_RUN -i | default 'manual') == "true" {
        let unreleased_version = $env | get -i NEW_VERSION 
        let unreleased_url = if ($unreleased_version | is-not-empty) { $'https://github.com/JadedBlueEyes/fendapp/releases/tag/($unreleased_version)' }
    } 

    # Iterate over each match and extract version, date, URL, and description
    let releases = $matches | each { |e|
        let version = $e | parse -r "<h2>\\s*<a[^>]*>(?P<version>.*?)</a>" | get -i version | get -i 0 
        let date = $e | parse -r "- (?P<date>[\\d]{4}(-[\\d]{2}){2})" | get -i date | get -i 0 | default (date now | format date "%F")
        let release_url = $e | parse -r "<h2>\\s*<a\\s+href=\"(?P<release_url>[^\"]*)\"" | get -i release_url | get -i 0
        let description = $e | parse -r '(a>|(h2>Unreleased changes))(:\s*(?P<tagline>[\W\w]*?))?(\s*- (?P<date>[\d]{4}(-[\d]{2}){2}))?<\/h2>\s*(?P<body>[\W\w]*)' | each { |d| $"<p>($version  | default 'Unreleased changes' )(if ($d.tagline | str trim) != "" { ': ' | append $d.tagline | str join '' } else {''} )</p>($d.body | str replace -r '<p\s*>\s*Full Changelog:([\W\w]*?)([\W\w]*?)<\/p>' '')" } | get -i 0

        # Print the extracted information
        {version: ($version | default "Unreleased") date: $date release_url: ($release_url) description: $description}
    }
    
    if ($env | get DRY_RUN -i | default 'manual') == "false" {
        return ($releases | filter {|e| ($e.version) != "Unreleased"})
    } else {
        return $releases
    }
}
