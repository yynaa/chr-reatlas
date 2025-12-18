# reatlaser-cli

cli tool for extracting chrs from a binary, or for rendering an atlas

## usage

### getting raw chr to a .png

```sh
# simple render
reatlaser-cli get gfx.bin output.png

# specify starting position and length
reatlaser-cli get gfx.bin output.png -p 0x1000 -l 0x810
```

### rendering atlas

```sh
# simple atlas render
reatlaser-cli get atlas.toml output.png
```
