# Dijkstra Algorithm for Godot



### What it does

Howdy !

This is a GDNative project for Godot game engine, that introduces Dijktra Map pathfinding node. It provides a much needed versatility currently absent from the build-in AStar pathfinding. Its main feature is the ability to populate the entire graph with shortest paths towards a given origin point. Total lenghs of shortest paths and directions can then be easily looked up for any point in the graph.

Common use cases include: pre-computing pathfinding for tower-defense games, RTS games and roguelikes; Listing available moves for turn-based games; aiding in movement-related AI behaviour. You can find more examples in [this amazing article](http://www.roguebasin.com/index.php?title=Dijkstra_Maps_Visualized).

The library is written in Rust programming language and performance should be comparable to C/C++ (aproximately 10-20x faster than GDScript).

Note that the API is now stable! Some features may be added over time.


### Installing

This repository contains pre-compiled binaries for Windows x64 and Linux x64. The project should just run after download on these platforms. The binaries may be slightly out-of-date, depending on branch in question. For other platforms you will have to compile the project yourself.

You will have to : 
* [install rust compiler](https://www.rust-lang.org/tools/install) and you will most likely have to install the dependencies described in [GDNative bindings for rust github](https://github.com/GodotNativeTools/godot-rust).

* Once you do that, open a terminal, navigate to the directory containing Cargo.toml and run "cargo build --release" command.

It will compile the DLL binary file (or equivalent) and put it into "res://target/release/" directory (first time around, it may take several minutes, because it has to automatically download and compile all the dependencies). 

* Preferably, you should move the binary to /Dijkstra_map_library/ directory to keep all the binaries organized.

* Open Godot and add path to the binary file into the "Dijkstra_map_library/dijkstra_map_library.tres" GDNativeLibrary resource. This resource tells godot which binary to use for which system. For more info see the [GDNative C example in Godot's documentation](https://docs.godotengine.org/en/stable/tutorials/plugins/gdnative/gdnative-c-example.html).

## Getting Started

After having followed *Installing* direction, open the godot project and open and run one of the demo scenes 
*  /Project Example/dragon_attack.tscn
*  /Project Example/Turn based.tscn
* /API demo/demo.tscn

toy around with it, the code of dragon_attack is heavily commented
mess with the exports variable in demo.tscn (click on the root node and tweak the values at the topof the inspector)

you can also look at the unit tests in Tests/unit/*

you should also look at our documentation in DOCUMENTATION.md file.

To use the Dijkstra Map in your own projects, you can copy the /Dijkstra_map_library/ directory to the root of your project (warning: putting it to subdirectories may crash godot, because paths in GDNativeLibrary resource are not relative).

## Features && HowTo's

#### Basic Behaviour
In godot project you create a new DijkstraMap Node.
* First you need to specify the graph by adding points (vetices) and connections between them (edges). Unlike build-in AStar, DijkstraMap does not keep positions of the points (it only ever refers to them by their ID) and the costs of the connections need to be explicitly specified. It is user's reponsibility to keep track of point's position.

you can do so manually with them `add_point` and `connect_points` methods 
or automatically with `add_*_grid` (`add_square_grid` or `add_hexagonal_grid` ...)

* once you've done that, you can enable or disable any points  you want from the pathfinding by passing its id to `enable_point` or `disable_point` (points are enabled by default).

* you then have to call `recalculate` method with appropriate arguments, by defaut, if you only have to pass an id or a PoolIntArray of id's, of the origin point. The method will calculate shortest paths from that origin point to every point in the graph.

* you can then access the information using various methods. Most notably `get_cost_map()` and `get_direction_map` which return a dictionary with keys being point's IDs and values being respective information about the length of the shortest path from that point or ID of the next point along the path.

* It is also possible to get a list of all points who's path's lenghs are withing given range, using the `get_all_points_with_cost_between()` method.

* You can also get the full shortest path from a given point using `get_shortest_path_from_point`

#### More recalculate flags
* if you look at the documentation you'll see you can optionnaly dive a dictionary to the recalculate method
'max cost' default to INF meaning the map will calculate for all points whose cost are below INF which means.. all points
but sometimes you dont need to recalculate all map, if you have a monster moving on a chess board that has a total of movement points equal to 20, it makes sense to set 'max cost' to 20, meaning we only calculate for the points the monster can actually reach
for more info go look at the documentation

#### The usefulness of terrain

* If you look at the documentation you might notice there's a mysterious feature called terrain (you can pass optionnal terrain id for add_point for instance) this serves the following purpose:

you want to share the same map (without redoing connections each turn) between a warrior that is slow on the forest but quick on the field, and a priest that is quick everywhere (priest are overpowered, you should really fix your game), but the map is very big and you cant afford to recalculate all the map after changing the connections each time.

the solution offered by terrain is this: you set the terrain id of forest to, lets say 0 but we'll call it CONST_TERRAIN_FOREST = 0 and CONST_TERRAIN_FIELD = 1.

you then set all the terrain id of your points via set_terrain_for_point
and then when you call recalculate, you pass a optional_params = a dictionary where key = "terrain weights" and value = a dictionary of terrains weigth
where key = CONST_TERRAIN_FOREST and the value is a mutiplier of the cost its gonna take for each connection where terrain is the terrain id specified.
for instance value = 1.0 is neutral (and default) but value = 5.0 means its gonna be harder to cross.

to be a little more precise I'll give an example: you have four points P(point_id,terrain_id) that are connected like such: 
P1 <----> P2 <----> P3 <----> P4 
if the terrain of P1 is default (-1 with cost 1.0 by default) and other points are all CONST_TERRAIN_FOREST and the cost between each connection is 1.0
you pass {CONST_TERRAIN_FOREST : 2.0} in optional
going from P1 to P2: cost is connection_cost(=1.0) * (cost_terrainP1(1.0)*cost_terrainP2(2.0)) /2 this connection between to different terrains was mutiplied by 1.5 total is 1.0 * 1.5 = 1.5
going from P2 to P3: cost is connection_cost(=1.0) * (cost_terrainP2(2.0)*cost_terrainP3(2.0)) /2 this connection between to different terrains was mutiplied by 2 total is 1.0 * 2.0 = 2.0
going from P1 to P4 cost 1.5 + 2*2.0 = 5.5

## Notes

Careful ! If you pass arguments of the wrong signature to the rust API, the game will not crash, if you're lucky and have a terminal open, it ight print an error there but not in godot! this issue can be avoided by using a gdscript wrapper
But it can lead to non trivial bugs, consider yourselves warned



A prettier wrapper in GDscript can be found at API demo/IDijkstra.gd
which is nice cause it offers autocompletion but a work in progress only



## Running the tests

If you're familiar with the gut API, you can launch the Gut.tscn and run some test

state of the test : currently few of them pass, its due to the person writing them (me) not having understood the API, but this will be fixed quickly


## Contributing

Open an issue before working on a feature, bugfix, unit tests, then we discuss it, then you can work on it (or let someone else) then do a pull request

Before doing that pull request, If you modified the rust code be sure you have build it "cargo build --release" and it still works! (the unit tests pass, dragon.tscn is running, the demo is running .

## TODO
find a way to compile it to all platforms in order to ship it via the godot asset store


## Acknowledgments
