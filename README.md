# murpapier

Simple cross platform rust wallpaper changer.
This app allows you to jump between several wallpapers at different hours.
In the `/wallpapers` you can find the Lakeside collection which particularly fits the application purpose.

In the most common use case, the murpapier app seeks for the files and hours in the config file (`config.toml`).
The murpapier has also an auto mode which automatically set an hour for each file in the wallapers directory.


## Dependencies
`cargo`, `rust`

## Set up

For \*nix systems, first fill in the `murpapier.service` your current desktop. 
Then execute the `install.sh` (it will set up the configuration files and add the service to the systemd/user).

## Customization

To customize the wallpapers and the time simply edit the `config.toml` file:
```toml
[[image]]
path="my_cool_wallpaper.png"
hour=10
min=0
```
