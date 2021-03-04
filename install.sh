#!/bin/sh

cargo build --release
echo "murpapier built"
cp ./target/release/murpapier /bin/
echo "murpapier added to /bin"
mkdir -p ~/.config/murpapier/wallpapers
cp ./wallpapers  ~/.config/murpapier/
cp ./config.toml ~/.config/murpapier/
echo "config files set up in ~/.config/murpapier/"
sudo cp ./murpapier.service /etc/systemd/user/
systemctl --user start murpapier
systemctl --user enable murpapier
echo "systemd well configured"
