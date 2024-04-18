export def main [level: string] {
    cargo release $level --no-publish --sign -x
}