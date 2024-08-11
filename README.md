# Fresnel Zone Calculator

This is a simple and intuitive application to calculate the Fresnel Zone for wireless communication. The application is built using Rust and `egui` for the graphical interface. It allows users to input the distance between two points and the frequency of the signal to calculate the Fresnel Zone, which is crucial for understanding potential signal interference.

## Screenshot

![Fresnel Zone Calculator in action](docs/ezgif-2-40bd29325b.gif)

## How to Use

1. **Launch the Application:** Run the application and you'll be presented with a simple interface.
2. **Enter the Distance:** Input the distance between the two points. You can switch between kilometers and miles using the radio buttons.
3. **Enter the Frequency:** Input the frequency of the signal in GHz. The maximum allowed frequency is 50 GHz.
4. **Calculate:** Click the "Calculate" button to see the radius of the first Fresnel Zone in meters.

## Build and Run

To build and run the application, you need to have Rust installed. Then, clone the repository and run the following commands:

```sh
cargo build --release
cargo run
```
