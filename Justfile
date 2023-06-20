plugins_dir := join(env_var('HOME'), ".config/zellij/plugins")
target := if env_var_or_default("RELEASE", "false") == "true" { "release" } else { "debug" } 

install:
  cargo build {{ if target == "release" { "--release" } else { "" } }}
  mkdir -p {{plugins_dir}}
  cp target/wasm32-wasi/{{target}}/zellij-tab-manager.wasm {{plugins_dir}}/zellij-tab-manager/zellij-tab-manager.wasm

try:
  just install
  cd .. && zellij
