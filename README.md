# Fresnel Zone Calculator


```
 _______ .______       _______     _______..__   __.  _______  __                                          
|   ____||   _  \     |   ____|   /       ||  \ |  | |   ____||  |                                         
|  |__   |  |_)  |    |  |__     |   (----`|   \|  | |  |__   |  |                                         
|   __|  |      /     |   __|     \   \    |  . `  | |   __|  |  |                                         
|  |     |  |\  \----.|  |____.----)   |   |  |\   | |  |____ |  `----.                                    
|__|     | _| `._____||_______|_______/    |__| \__| |_______||_______|                                    
                                                                                                           
  ______     ___       __        ______  __    __   __          ___   .___________.  ______   .______      
 /      |   /   \     |  |      /      ||  |  |  | |  |        /   \  |           | /  __  \  |   _  \     
|  ,----'  /  ^  \    |  |     |  ,----'|  |  |  | |  |       /  ^  \ `---|  |----`|  |  |  | |  |_)  |    
|  |      /  /_\  \   |  |     |  |     |  |  |  | |  |      /  /_\  \    |  |     |  |  |  | |      /     
|  `----./  _____  \  |  `----.|  `----.|  `--'  | |  `----./  _____  \   |  |     |  `--'  | |  |\  \----.
 \______/__/     \__\ |_______| \______| \______/  |_______/__/     \__\  |__|      \______/  | _| `._____|
                                                                                                           
```

This is a simple and intuitive application to calculate the Fresnel Zone for wireless communication. The application is built using Rust and `egui` for the graphical interface. It allows users to input the distance between two points and the frequency of the signal to calculate the Fresnel Zone, which is crucial for understanding potential signal interference.

## Screenshot

![Fresnel Zone Calculator in action](docs/ezgif-2-40bd29325b.gif)

## How to Use

1. **Launch the Application:** Run the application and you'll be presented with a simple interface.
2. **Enter the Distance:** Input the distance between the two points. You can switch between kilometers and miles using the radio buttons.
3. **Enter the Frequency:** Input the frequency of the signal in GHz. The maximum allowed frequency is 50 GHz.
4. **Calculate:** Click the "Calculate" button to see the radius of the first Fresnel Zone in meters.


## Installation

You can easily install the Fresnel Zone Calculator by running the following command:

```sh
curl -sSf https://raw.githubusercontent.com/fzaca/fresnel_calculator/master/install.sh | sh
```

To build and run the application, you need to have Rust installed. Then, clone the repository and run the following commands:

```sh
cargo build --release
cargo run
```
