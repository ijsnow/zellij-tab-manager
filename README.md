# Zellij Tab Manager

## Installation/configuration

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
just try
```

What `just try` does the following:

1. Build the plugin.
2. Copy it to `~/.config/zellij/plugins/zellij-tab-manager/zellij-tab-manager.wasm`
3. Navigate to the parent directory of the cloned repo and run `zellij`.

The plugin assumes the directory you cloned the directory to is a directory you have most of your local repositories in. It navigates to this "workspace directory" because:

1. It will now be the `/host` directory available to the plugin and each directory entry is a possible tab.
  a. A future todo would be to make this compatible. Maybe assuming you start zellij from `$HOME` you can change `cwd` to whatever.
2. It looks for `/host/zellij-tab-manager/config/layouts/tab/default.kdl` for a layout template file. It currently supports replacement of `$name` with the name of the selected directory.

## Problems/Questions

1. What directory is `/data` mapped to? Thought it would be `~/.config/zellij/plugins/zellij-tab-manager/` for the location described above.
