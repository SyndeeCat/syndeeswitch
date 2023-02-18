# syndeeswitch
Theme switcher for linux. It copies file content from `*.{--theme}*` to `*.*` located in `{--dir}`.

## Usage:
```bash
syndeeswitch -t light -d ~/.config
```

## For what reason?
Let's say you have a config that has a dark theme color palette. 
But during daylight hours, such elements will be dim. 
For such a case, you can keep two configs: one for a light theme, the other for a dark one.

For example, `~/.config/alacritty/colors.light.yaml` with light color palette and `~/.config/alacritty/colors.dark.yaml` for dark one.

So, after
```bash
syndeeswitch -t light -d ~/.config
```

`~/.config/alacritty/colors.light.yaml` will be copied to `~/.config/alacritty/colors.yaml` where your config should be.

## Roadmap:
- [X] minimal valuable product
- [ ] add switching gtk themes
