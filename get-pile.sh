#/bin/sh

SHARE="~/.local/share"
PILE_HOME="$SHARE/pile"

$(mkdir -p "$SHARE" && cd "$SHARE" && git clone "https://github.com/chunibio/pile")
chmod +x "PILE_HOME/pile.sh"
ln -s "PILE_HOME/pile.sh" "/usr/bin/pile"
