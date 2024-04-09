A rust binary to use for ZSH Prompts (and possibly others)

![image](https://github.com/zschreur/jj-zsh-theme/assets/51675520/553960a6-baa0-4814-9079-36abe174f18e)

## Setup
### Install
- Installation depends on rust. You can follow the steps to download rust here - https://rustup.rs/
- Use cargo install to build off of `main`
```
cargo install --git https://github.com/zschreur/jj-zsh-theme.git --locked
```

### Adding the theme
- Create a zsh-theme file
```
echo "PROMPT='\$(jj-zsh-theme)'" > ~/.oh-my-zsh/custom/themes/jj.zsh-theme
```
- Now set `ZSH_THEME="jj"` in your `.zshrc` file
