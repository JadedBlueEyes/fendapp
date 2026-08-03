export def main [out_dir: string] {
    let binaries_dir = (cargo build --release --message-format=json | lines | each {|| from json} | where {|i| $i.reason == "compiler-artifact" } | where {|i| $i.executable | is-not-empty } | last | get executable | path dirname)
    cargo-packager -vvv --release --binaries-dir $binaries_dir -o $out_dir --formats all
}
