extends TileMap

var dijkstra_map = DijkstraMap.new()
var position_to_id={}
var id_to_position={}

func _exit_tree():
	dijkstra_map.free()

func _ready():
	#we need to initialize the dijkstra map with appropriate graph for pathfinding.
	#we will use "add_square_grid()" method to do this
	var rect=self.get_used_rect()
	#first argument is initial offset for the point IDs, second is the rectangle
	#third argument is terrain_id. We can ignore that one, since will will specify terrain later
	#last two arguments are costs for orthogonal/diagonal movement.
	#the method will return a dictionary of position to ID.
	position_to_id=dijkstra_map.add_square_grid(0,rect,-1,1.0,1.4)
	
	#now we will itterate through the positions and change the terrain to appropriate value
	for pos in position_to_id.keys():
		var id=position_to_id[pos]
		var terrain_id=self.get_cellv(pos) #we will simply use the IDs of the tiles in tileset
		#dijkstra map only references points by their ID. It is oblivious to their actual position.
		dijkstra_map.set_terrain_for_point(id,terrain_id)
		#we also make id_to_position dictionary for convenience
		id_to_position[id]=pos
	
	#now we prompt the knight to recalculate his access area
	var knight=get_node("knight")
	knight.stopped()


func redraw_movement_access(position,max_cost,terrain_weights):
	#here we recalculate the DijkstraMap to reflect movement of specific unit
	var pos=self.world_to_map(position)
	var id=position_to_id[pos]
	dijkstra_map.recalculate(id,{"terrain weights":terrain_weights})
	
	#now highlight the tiles
	#first we get all tiles with cost below "max_cost"
	var point_ids=Array(dijkstra_map.get_all_points_with_cost_between(0.0,max_cost))
	
	#now we highlight all the tiles in the highlight tilemap
	var highlight=get_node("highlight")
	highlight.clear()
	for id in point_ids:
		pos=id_to_position[id]
		highlight.set_cellv(pos,4)

func _unhandled_input(event):
	if event is InputEventMouseButton and event.pressed==false:
		var pos=self.world_to_map(get_local_mouse_position())
		#check if clicked point is within walking range (ie. if its highlighted)
		var highlight=get_node("highlight")
		if highlight.get_cellv(pos)!=-1:
			#get the shortest path form the DijkstraMap
			#and translate it into positions
			#note: the path is already pre-calculated. This method only fetches the result.
			#all of the actual pathfinding was performed by the "recalculate()" method earlier.
			var path_ids=dijkstra_map.get_shortest_path_from_point(position_to_id[pos])
			
			var path=[]
			#note: the selected point is not in the path
			path.push_back(self.map_to_world(pos)+self.cell_size*0.5)
			for id in path_ids:
				path.push_back(self.map_to_world(id_to_position[id])+self.cell_size*0.5)
			
			#now give the path to the knight
			var knight=get_node("knight")
			knight.path=path
			#change the highlight for target point only
			highlight.clear()
			highlight.set_cellv(pos,4)

