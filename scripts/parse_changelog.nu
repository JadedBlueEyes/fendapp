# let html_content = (pandoc --from markdown --to html5 --section-divs CHANGELOG.md )
export def parse_changelog_html [html_content: string] {
    let matches = $html_content | parse -r "<section\\s+id=\"[a-z0-9\\-]*\"\\s+class=\"level2\"\\s*>([\\W\\w]*?)</section>" | get capture0

    # Iterate over each match and extract version, date, URL, and description
    let releases = $matches | each { |e|
        let version = $e | parse -r "<h2>\\s*<a[^>]*>(?P<version>.*?)</a>" | get -i version | get -i 0 
        let date = $e | parse -r "- (?P<date>[\\d]{4}(-[\\d]{2}){2})" | get -i date | get -i 0 | default (date now | format date "%F")
        let release_url = $e | parse -r "<h2>\\s*<a\\s+href=\"(?P<release_url>[^\"]*)\"" | get -i release_url | get -i 0
        let description = $e | parse -r '>(:\s*(?P<tagline>[\W\w]*?))?((\s*- (?P<date>[\d]{4}(-[\d]{2}){2}))|(Unreleased changes))<\/h2>\s*(?P<body>[\W\w]*)' | each { |d| $"<p>($version | default 'Unreleased changes' )(if ($d.tagline | str trim) != "" { ': ' | append $d.tagline | str join '' } else {''} )</p>($d.body | str replace -r '<p\s*>\s*Full Changelog:([\W\w]*?)([\W\w]*?)<\/p>' '')" } | get -i 0

        # Print the extracted information
        {version: ($version | default "Unreleased") date: $date release_url: $release_url description: $description}
    }
    return $releases
}