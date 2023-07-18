using System;
using Godot;
using Godot.Collections;
using Object = Godot.Object;

public class DijkstraMap : Node
{
    private Object _dijkstraMap; 

    public DijkstraMap()
    {
        var dijkstraMapScript = GD.Load("res://addons/dijkstra-map/Dijkstra_map_library/nativescript.gdns") as NativeScript;
        _dijkstraMap = dijkstraMapScript?.New() as Object;
        if (_dijkstraMap is null) throw new ArgumentNullException($"{nameof(_dijkstraMap)} cannot be null.");
    }

    public Dictionary<Vector2, int> AddSquareGrid(Rect2 bounds, int terrain = -1, float orthCost = 1f,
        float diagCost = float.PositiveInfinity)
    {
        var dictionary = _dijkstraMap.Call("add_square_grid", bounds, terrain, orthCost, diagCost) 
            as Dictionary;
        return new Dictionary<Vector2, int>(dictionary);
    }

    public void Recalculate(int pointId, Dictionary<string, object> options)
    {
        _dijkstraMap.Call("recalculate", pointId, options);
    }
    
    public void Recalculate(Array<int> pointIds, Dictionary<string, object> options)
    {
        _dijkstraMap.Call("recalculate", pointIds, options);
    }

    public Dictionary<int, float> GetCostMap()
    {
        var dictionary = _dijkstraMap.Call("get_cost_map") as Dictionary;
        return new Dictionary<int, float>(dictionary);
    }

    public Dictionary<int, int> GetDirectionMap()
    {
        var dictionary = _dijkstraMap.Call("get_direction_map") as Dictionary;
        return new Dictionary<int, int>(dictionary);
    }
    
    public void SetTerrainForPoint(int pointId, int terrain)
    {
        _dijkstraMap.Call("set_terrain_for_point", pointId, terrain);
    }
}