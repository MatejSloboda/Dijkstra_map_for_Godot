extends TileMap

# warning-ignore:unused_class_variable
var dmap=preload("res://dijkstra_map.gdns")
var dijkstra_map_for_pikemen
var dijkstra_map_for_archers

var point_id_to_position={}
var point_position_to_id={}
var speed_modifiers={}

# Called when the node enters the scene tree for the first time.
func _ready():
	#we create our Dijkstra maps. We will need 2 - one for archers, one for pikemen
	dijkstra_map_for_archers=DijkstraMap.new()
	dijkstra_map_for_pikemen=DijkstraMap.new()
	
	#first we must add all points and connections to the dijkstra maps
	#this only has to be done once, when project loads
	
	#we collect all walkable tiles from the tilemap
	var walkable_tiles=[]
	for tilename in ["grass","bushes","road"]:
		var tile_id=tile_set.find_tile_by_name(tilename)
		walkable_tiles=walkable_tiles+get_used_cells_by_id(tile_id)
	
	#DijkstraMap only ever references points by their unique ID.
	#It does not know about their actual position or even what they represent.
	#We will have to keep dictionaries to lookup position by ID and vice versa
	point_id_to_position.clear()
	point_position_to_id.clear()
	#now we insert the points
	var id=0
	for pos in walkable_tiles:
		id=id+1
		point_id_to_position[id]=pos
		point_position_to_id[pos]=id
		dijkstra_map_for_archers.add_point(id)
		dijkstra_map_for_pikemen.add_point(id)
		
	#now we need to connect the points with connections
	#each connection has a source point, target point and a cost
	
	#since we have multiple tile types and want different speed modifiers for them,
	#we will have to reflect that in costs of connections.
	speed_modifiers[tile_set.find_tile_by_name("grass")]=1.0
	speed_modifiers[tile_set.find_tile_by_name("bushes")]=0.5
	speed_modifiers[tile_set.find_tile_by_name("road")]=1.5
	
	var orthogonal=[Vector2.DOWN,Vector2.UP,Vector2.LEFT,Vector2.RIGHT]
	var diagonal=[Vector2.DOWN+Vector2.LEFT,Vector2.UP+Vector2.LEFT,
	Vector2.DOWN+Vector2.RIGHT,Vector2.UP+Vector2.RIGHT]
	
	for pos in walkable_tiles:
		#NOTE: costs are a measure of time. they are distance/speed
		var cost_of_current_tile=1.0/speed_modifiers[get_cellv(pos)]
		var id_of_current_tile=point_position_to_id[pos]
		#we loop through orthogonal tiles
		for offset in orthogonal:
			#note neighbour might not exist, so we need some default values indicating absence
			var cost_of_neighbour=1.0/speed_modifiers.get(get_cellv(pos+offset),INF)
			var id_of_neighbour=point_position_to_id.get(pos+offset,-1) #ID=-1 represents absent point
			#we will set the cost as average of costs of tiles
			var cost=(cost_of_current_tile+cost_of_neighbour)/2.0
			#NOTE: if points dont exist, .connect_points does nothing
			dijkstra_map_for_archers.connect_points(id_of_current_tile,id_of_neighbour,cost,false)
			dijkstra_map_for_pikemen.connect_points(id_of_current_tile,id_of_neighbour,cost,false)
		
		#we do the same for diagonal tiles, except cost is further multiplied by sqrt(2)
		for offset in diagonal:
			var cost_of_neighbour=1.0/speed_modifiers.get(get_cellv(pos+offset),INF)
			var id_of_neighbour=point_position_to_id.get(pos+offset,-1)
			var cost=sqrt(2.0)*(cost_of_current_tile+cost_of_neighbour)/2.0
			dijkstra_map_for_archers.connect_points(id_of_current_tile,id_of_neighbour,cost,false)
			dijkstra_map_for_pikemen.connect_points(id_of_current_tile,id_of_neighbour,cost,false)
	
	
	#now that points are added and properly connected, we can calculate the dijkstra maps
	recalculate_dijkstra_maps()
	
			

func recalculate_dijkstra_maps():
	#where is the dragon?
	var dragon=point_position_to_id.get(world_to_map(get_node("dragon").position),0)
	
	#we want pikemen to charge the dragon head on
	#We recalculate the DijkstraMap.
	#As target we set the ID of the point closest to dragon.
	#We want the DijkstraMap to calculate as far as it can, so we pass INF.
	#We want the dragon to be a target, not a source, so we pass false.
	dijkstra_map_for_pikemen.recalculate_for_target(dragon,INF,false)
	#now the map has recalculated for pikemen and we can access the data.
	
	#we want archers to stand at safe distance from the dragon, but within firing range.
	#first we recalculate their Dijkstra map with dragon as the source.
	#we calculate only distance 7 since we dont need more.
	dijkstra_map_for_archers.recalculate_for_target(dragon,INF,true)
	#now we get IDs of all points safe distance from dragon, but within firing range
	var stand_over_here=dijkstra_map_for_archers.get_all_points_with_cost_between(3.0,4.0)
	
	#and we pass those points as new targets for the archers to walk towards
	dijkstra_map_for_archers.recalculate_for_targets(stand_over_here,INF,false)
	#BTW yes, Dijkstra map works for multiple target points too.
	#The path will simply lead towards the nearest target point.
	
	
func get_speed_modifier(pos):
	return speed_modifiers.get(get_cellv(world_to_map(pos)),0.5)

#given position of a pikeman,
#this method will look up the indented direction of movement for the pikeman.
func get_direction_for_pikeman(pos):
	var map_coords=world_to_map(pos)
	#we look up in the Dijkstra map where the pikeman should go next
	var target_ID=dijkstra_map_for_pikemen.get_direction_at_point(point_position_to_id[map_coords])
	#if dragon is inaccessible from current position, then Dijkstra map spits out -1
	if target_ID==-1:
		return Vector2(0,0)
	var target_coords=point_id_to_position[target_ID]
	return map_coords.direction_to(target_coords)

func get_direction_for_archer(pos):
	var map_coords=world_to_map(pos)
	#we look up in the Dijkstra map where the archer should go next
	var target_ID=dijkstra_map_for_archers.get_direction_at_point(point_position_to_id[map_coords])
	#if dragon is inaccessible from current position, then Dijkstra map spits out -1
	if target_ID==-1:
		return Vector2(0,0)
	var target_coords=point_id_to_position[target_ID]
	return map_coords.direction_to(target_coords)


func _unhandled_input(event):
	if event is InputEventMouseButton:
		var pos=get_local_mouse_position()
		var dragon=get_node("dragon")
		dragon.position=pos
		recalculate_dijkstra_maps()