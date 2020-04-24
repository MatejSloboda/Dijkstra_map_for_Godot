# Dijkstra_map_for_Godot

It doesn't contain the compiled rust library, so it will not work off the bat.

You will have to install rust compiler and you will most likely have to install the dependencies described in GDNative bindings for rust github.
Once you do that, open a terminal, navigate to the directory and run "cargo build --release" command.
It will compile the dll and put it into "res://target/release/" directory (first time around, it may take several minutes, because it has to automatically download and compile all the dependencies).
If you are on x64 windows, the example should just work from there. Otherwise, you will have to update the "dijkstra_map_library.tres" file. (It tells godot which file to use for the library for each system.)

To do so, click on that file in the godot editor, and fill the section corresponding your OS with "res://target/release/libdijkstra_map_for_godot.so" for instance

The following example demonstrates the use of Dijksta_map with heavily commented code.
