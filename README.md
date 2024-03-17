# yesno

The `yesno` command prompts the user with yes/no quetion and exits with status code 0 or 1 accordingly. This can be useful for situations like:

```
cmd1 && yesno && cmd2
```

One personal use case is:
```
brew update &>/dev/null && brew outdated && yesno && brew upgrade
```


install
```
cargo build --release
sudo cp target/release/yesno /usr/local/bin
```
