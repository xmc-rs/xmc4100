
Remove-Item .\src -Recurse -Force
svd2rust -i .\XMC4100.svd --ident-formats-theme legacy
New-Item src -ItemType Directory | Out-Null
form -i .\lib.rs -o .\src
cargo fmt
Remove-Item .\lib.rs -Force
