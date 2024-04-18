

use parse_changelog.nu parse_changelog_html

let html_content = (pandoc --from markdown --to html5 --section-divs --strip-comments CHANGELOG.md )
let changelog = parse_changelog_html $html_content

def html_to_xml_tree [html_content: string] {
    $html_content | parse -r '(?P<text>[\W\w]*?)((<(?P<tag>[A-Za-z][A-Za-z0-9\-]*)\b[^>]*>(?P<content>[\W\w]*?)<\/\k'tag'>)|$)'| reduce -f [] { |e, acc|

        if not ( $e.text | is-empty) and not ( $e.tag | is-empty) {
            ($acc | append $e.text | append {tag: $e.tag content: (html_to_xml_tree ($e.content | default ""))})
        } else if not ( $e.text | is-empty) {
            ($acc | append $e.text)
        } else if not ( $e.tag | is-empty) {
            ($acc | append {tag: $e.tag content: (html_to_xml_tree ($e.content | default ""))})
        } else  {
            $acc
        }
        
        } 
}

{tag: releases content: ($changelog | each {|e| {tag: release attributes: {version: $e.version date: $e.date } content: [(if $e.release_url != null {{tag: url attributes: {type: "details"} content: [$e.release_url] }} else {""}), {tag: description content: (html_to_xml_tree $e.description)}] } }) } | to xml --indent 2 | save ./assets/link.ellis.jade.fendapp.releases.xml -f
# html_to_xml_tree ($changelog | get 0.description)
# html_to_xml_tree " hi <p>hi</p>"