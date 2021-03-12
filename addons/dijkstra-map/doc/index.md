Implementation of [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm) in Rust.

`DijkstraMap` is a general-purpose pathfinding class. It is intended
to cover functionality that is currently absent from build-in
[AStar] pathfinding class. Its main purpose is to do bulk
pathfinding by calculating shortest paths between given point and
all points in the graph. It also allows viewing useful information
about the paths, such as their length, listing all paths with
certain length, etc.

Just like [AStar], `DijkstraMap` operates on directed weighted
graph. To match the naming convention with [AStar], vertices are
called points and edges are called connections. Points are always
referred to by their unique [integer](https://docs.godotengine.org/en/stable/classes/class_int.html) ID. Unlike [AStar],
`DijkstraMap` does not store information about their real positions.
Users have to store that information themselves, if they want it;
for example, in a [Dictionary].
# Classes:
- [DijkstraMap](./DijkstraMap.md)

[AStar]: https://docs.godotengine.org/en/stable/classes/class_astar.html
[Dictionary]: https://docs.godotengine.org/en/stable/classes/class_dictionary.html