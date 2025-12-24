```
                                   __
  ___  __  __ ___ _ __ ___   ___  / /_ ___
 / _ \/ / / // _ \ '_ ` _ \ / _ \/ __// _ \
/ / / / /_/ /| (_| | | | | | (_) | |_ |  __/
/_/ /_/\__, / \__,_| |_| |_|\___/ \__/ \___|
      /____/
      /\_____/\
     /  o   o  \
    ( ==  ^  == )
     )         (
    (           )
   ( (  )   (  ) )
  (__(__)___(__)__)
```

# nyamote

control your pc from your phone :3

## features

- touch pad to move mouse
- left/right click buttons
- scroll up/down
- keyboard mode with text input
- arrow keys, tab, backspace, enter, esc, space
- works on wayland & x11

## requirements

- ydotool (for input simulation)

```bash
# arch
sudo pacman -S ydotool

# enable daemon
systemctl --user enable --now ydotool
```

## install

```bash
cargo install --git https://github.com/0xdilo/nyamote
```

or with upx compression:
```bash
git clone https://github.com/0xdilo/nyamote
cd nyamote
make install
```

## usage

```bash
nyamote           # default port 8888
nyamote 9000      # custom port
```

then open `http://<your-ip>:8888` on your phone

## license

do whatever u want with it lol
