# idotmatrix

Rust implementation of a driver for an iDotMatrix bluetooth display

based on https://github.com/derkalle4/python3-idotmatrix-client

the driver is in lib.rs, an example with a command line interface is provided and ready for use

## Usage

```
> cargo run -q --example cli -- --help
Usage: cli [OPTIONS]

Options:
      --screen-on                              
      --screen-off                             
      --set-pixel <SET_PIXEL>                  Pixel in format x,y,#ffffff
      --image-mode <IMAGE_MODE>                
      --upload-png <UPLOAD_PNG>                Path to png file
      --full-screen-color <FULL_SCREEN_COLOR>  Color in hex format, e.g. #ffffff
      --screen-brightness <SCREEN_BRIGHTNESS>  Brightness in percent, e.g. 100
      --countdown-start <COUNTDOWN_START>      Countdown in seconds
      --countdown-cancel                       
      --countdown-pause                        
      --countdown-resume                       
      --color-hue                              Continuously change color demo
  -h, --help                                   Print help
  -V, --version                                Print version

```

## Example

```
cargo run --example cli -- --upload-png demo_32.png
```

```
cargo run --example cli -- --full-screen-color "#00ff00"
```

```
cargo run --example cli -- --set-pixel "2,2,#00ff00"
```
