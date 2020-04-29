# Dijkstra Algorithm for Godot



### What it does

Howdy !

This is a gdnative project to bring the Dikstra algorithm into godot! It serves as pathfinding in a graph and allow to find all path leading to a point source (or all points accessible from a point target) which is more powerful than the A* algorithm

Note that we are in early development and the API is not stable, its still regularly edited!

Also note that other uses can be found in this amazing article : http://www.roguebasin.com/index.php?title=Dijkstra_Maps_Visualized


### Installing


It doesn't contain the compiled rust library, so it will not work off the bat.

You will have to : 
* install rust compiler and you will most likely have to install the dependencies described in GDNative bindings for rust github.

* Once you do that, open a terminal, navigate to the directory containing Cargo.toml and run "cargo build --release" command.

It will compile the rust executable and put it into "res://target/release/" directory (first time around, it may take several minutes, because it has to automatically download and compile all the dependencies). 

If you are on x64 windows, the example should just work from there. Otherwise, you will have to update the "dijkstra_map_library.tres" file. (It tells godot which file to use for the library for each system.)

## Getting Started

After having followed *Installing* direction, open the godot project and open and run one of the demo scenes 
*  /Project Example/dragon_attack.tscn
* /API demo/demo.tscn

toy around with it, the code of dragon_attack is heavily commented
the code of demo.tscn is still a little immature and shouldnt be taken as example ... (especially the ugly wrapper I put there)

### Features

TODO list features

### Notes

Careful ! If you pass arguments of the wrong signature to the rust API, the game will not crash, if you're lucky and have a terminal open, it ight print an error there but not in godot! this issue can be avoided by using a gdscript wrapper
But it can lead to non trivial bugs, consider yourselves warned



A prettier wrapper in GDscript can be found at ???/IDijkstra.gd
which is nice cause it offers autocompletion but a work in progress only



## Running the tests

If you're familiar with the gut API, you can launch the Gut.tscn and run some test

state of the test : currently few of them pass, its due to the person writing them (me) not having understood the API, but this will be fixed quickly


## Contributing

Open an issue before working on a feature, bugfix, unit tests, then we discuss it, then you can work on it (or let someone else) then do a pull request

Before doing that pull request, If you modified the rust code be sure you have build it "cargo build --release" and it still works! (the unit tests pass **are in the same or better state than before, I'm wworking on fixing them** , dragon.tscn is running, the demo is running **currently it is broken no matter what, I'm working on it ;)**.


## Acknowledgments
