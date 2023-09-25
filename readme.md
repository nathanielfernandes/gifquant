# gifquant

simple cli tool for converting images to gifs, allowing for reducing/changing the color palette of the output gif.

just needed a quick and better way to convert my asesprite animations to gifs, so i made this.

palettes need to be in the [gpl](https://www.gimp.org/docs/userfaq.html#q1-2) format.

all art in the `images` folder are my own.

### Options

- `-c, --colors <colors>`: number of colors to use in the output gif (default: 256)
- `-p, --palette <palette>`: path to a gpl file to use as the color palette
- `-o, --output <output>`: path to the output gif file
- `-h, --help`: display help for command
- `-d, --delay <delay>`: delay between frames in the output gif (default: 100(ms))
- `-l, --loop <loop>`: number of times to loop the output gif (default: 0 (infinite))
- `-i, --interlaced`: whether or not to use interlacing in the output gif (default: false)
- `-s, --speed <speed>`: quantization speed (1-30) (default: 1)

## Examples

### use max (256) colors

```
gifquant images -o full.gif
```

<img src="./assets/full.gif" width="512">

### use 8 colors

```
gifquant images -c 8 -o c8.gif
```

<img src="./assets/c8.gif" width="512">

### use 16 colors

```
gifquant images -c 16 -o c16.gif
```

<img src="./assets/c16.gif" width="512">

### use 32 colors

```
gifquant images -c 32 -o c32.gif
```

<img src="./assets/c32.gif" width="512">

## use external color palette

```
gifquant images -p palettes/aap-64.gpl -o aap-64.gif
```

<img src="./assets/aap-64.gif" width="512">
