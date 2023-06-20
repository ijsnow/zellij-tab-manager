# Zellij Tab Manager

## Getting Started

```
// ~/.config/zellij/config.kdl

keybinds {
    // ...

    shared {
        bind "Ctrl y" { 
            LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellij-tab-manager/zellij-tab-manager.wasm" {
                floating true
            } 
        }
    }
}
```

```shell
git clone git@github.com:ijsnow/zellij-tab-manager.git
cd zellij-tab-manager
just install
```

## Problems/Questions

1. What directory is `/data` mapped to? Thought it would be `~/.config/zellij/plugins/zellij-tab-manager/` for the location described above.
