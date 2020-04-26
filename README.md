# Dijkstra Algorithm for Godot



### What it does



### Installing


It doesn't contain the compiled rust library, so it will not work off the bat.

You will have to : 
* install rust compiler and you will most likely have to install the dependencies described in GDNative bindings for rust github.

* Once you do that, open a terminal, navigate to the directory containing Cargo.toml and run "cargo build --release" command.

It will compile the rust executable and put it into "res://target/release/" directory (first time around, it may take several minutes, because it has to automatically download and compile all the dependencies). 

## Getting Started

After having followed *Installing* direction, open the godot project and open and run one of the demo scenes 
*  /Project Example/dragon_attack.tscn
* /API demo/demo.tscn

toy around with it, the code of dragon_attack is heavily commented
the code of demo.tscn is still a little immature and shouldnt be taken as example ... (especially the ugly wrapper I put there)

### Notes

Careful ! If you pass arguments of the wrong signature to the rust API, the game will not crash, if you're lucky and have a terminal open, it ight print an error there but not in godot! this issue can be avoided by using a gdscript wrapper
But it can lead to non trivial bugs, consider yourselves warned



A prettier wrapper in GDscript can be found at ???/IDijkstra.gd
which is nice cause it offers autocompletion but a work in progress only



## Running the tests

If you're familiar with the gut API, you can launch the Gut.tscn and run some test


## Contributing

## Authors


## License


## Acknowledgments
