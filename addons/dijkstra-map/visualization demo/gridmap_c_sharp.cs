using Godot;
using System;
using Godot.Collections;
using Object = Godot.Object;

public class gridmap_c_sharp : TileMap
{
    private readonly DijkstraMap _dijkstraMap = new DijkstraMap();
    private readonly Dictionary<int, Vector2> _idToPos = new Dictionary<int, Vector2>();
    private Dictionary<Vector2, int> _posToId = new Dictionary<Vector2, int>();
    private int _tileDraw = 0;
    private readonly Dictionary<int, float> _terrainWeights = new Dictionary<int, float>
    {
        { 0, 1f }, { 1, 4f }, { 2, float.PositiveInfinity }, { 3, 1f }
    };
    private bool _dragging = false;

    public override void _Ready()
    {
        GetNode("../terrain selection").Connect("item_selected", this, nameof(OnTerrainSelectionItemSelected));
        
        var @event = new InputEventMouseButton();
        @event.ButtonIndex = (int)ButtonList.Left;
        if (InputMap.HasAction("left_mouse_button") is false)
        {
            InputMap.AddAction("left_mouse_button");
        }
        InputMap.ActionAddEvent("left_mouse_button", @event);

        var bmp = new Rect2(0, 0, 23, 19);
        _posToId = _dijkstraMap.AddSquareGrid(bmp);
        foreach (var posToId in _posToId)
        {
            _idToPos[posToId.Value] = posToId.Key;
        }

        UpdateTerrainIds();
        Recalculate();
    }

    private void Recalculate()
    {
        var targets = new Array<Vector2>(GetUsedCellsById(3));
        var targetIds = new Array<int>();
        foreach (var pos in targets)
        {
            targetIds.Add(_posToId[pos]);
        }
        _dijkstraMap.Recalculate(targetIds, new Dictionary<string, object>
        {
            { "terrain_weights", _terrainWeights }
        });
        
        // Visualize
        var costs = _dijkstraMap.GetCostMap();
        var costGrid = GetNode<TileMap>("costs");
        costGrid.Clear();

        foreach (var id in costs.Keys)
        {
            var cost = (int)costs[id];
            cost = Mathf.Min(32, Mathf.Max(0, cost));
            costGrid.SetCell((int)_idToPos[id].x, (int)_idToPos[id].y, 0, false, false, false, 
                new Vector2(cost, 0));
        }

        var dirToTile = new Dictionary<Vector2, int>
        {
            { new Vector2(1, 0), 0 },
            { new Vector2(1, -1), 1 },
            { new Vector2(0, 1), 2 },
            { new Vector2(1, 1), 3 },
            { new Vector2(-1, 0), 4 },
            { new Vector2(-1, 1), 5 },
            { new Vector2(0, -1), 6 },
            { new Vector2(-1, -1), 7 },
        };

        var dirIds = _dijkstraMap.GetDirectionMap();
        var dirGrid = GetNode<TileMap>("directions");
        dirGrid.Clear();

        foreach (var id1 in dirIds.Keys)
        {
            var pos = _idToPos[id1];
            var vec = _idToPos.ContainsKey(dirIds[id1]) 
                ? _idToPos[dirIds[id1]] - pos 
                : new Vector2(float.NaN, float.NaN) - pos;
            var tile = dirToTile.ContainsKey(vec)
                ? dirToTile[vec]
                : float.NaN;
            if (float.IsNaN(tile) is false)
            {
                dirGrid.SetCell((int)pos.x, (int)pos.y, 1, false, false, false, 
                    new Vector2(tile, 0));
            }
        }
    }

    private void UpdateTerrainIds()
    {
        foreach (var id in _idToPos.Keys)
        {
            var pos = _idToPos[id];
            _dijkstraMap.SetTerrainForPoint(id, GetCellv(pos));
        }
    }

    private void OnTerrainSelectionItemSelected(int index)
    {
        _tileDraw = index;
    }
    
    public override void _UnhandledInput(InputEvent @event)
    {
        if (@event.IsActionPressed("left_mouse_button")) _dragging = true;
        if (@event.IsActionReleased("left_mouse_button")) _dragging = false;

        if ((@event is InputEventMouseMotion || @event is InputEventMouseButton) && _dragging)
        {
            var pos = GetLocalMousePosition();
            var cell = WorldToMap(pos);
            if (cell.x >= 0 && cell.x < 23 && cell.y >= 0 && cell.y < 19)
            {
                SetCellv(cell, _tileDraw);
                _dijkstraMap.SetTerrainForPoint(_posToId[cell], _tileDraw);
                Recalculate();
            }
        }
    }
}

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