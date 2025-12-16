namespace AdventOfCode.Core.Utilities;

public static class GraphUtilities
{
    public static List<List<T>> GetAllConnectedComponents<T>(
        Dictionary<T, List<T>> adjacencyList) where T : notnull
    {
        var visited = new HashSet<T>();

        return adjacencyList
            .Keys
            .Where(node => !visited.Contains(node))
            .Select(node => GetConnectedComponent(adjacencyList, visited, node))
            .ToList();
    }

    private static List<T> GetConnectedComponent<T>(
        Dictionary<T, List<T>> adjacencyList,
        HashSet<T> visited,
        T start) where T : notnull
    {
        var component = new List<T>();
        var stack = new Stack<T>();

        stack.Push(start);
        visited.Add(start);

        while (stack.Count > 0)
        {
            var node = stack.Pop();
            component.Add(node);

            foreach (var neighbor in adjacencyList[node])
                if (!visited.Contains(neighbor) && adjacencyList.ContainsKey(neighbor))
                {
                    visited.Add(neighbor);
                    stack.Push(neighbor);
                }
        }

        return component;
    }

    public static List<T> GetConnectedComponent<T>(
        Dictionary<T, List<T>> adjacencyList,
        T start) where T : notnull
    {
        var component = new List<T>();
        var stack = new Stack<T>();
        var visited = new HashSet<T>();

        stack.Push(start);
        visited.Add(start);

        while (stack.Count > 0)
        {
            var node = stack.Pop();
            component.Add(node);

            foreach (var neighbor in adjacencyList[node])
                if (!visited.Contains(neighbor) && adjacencyList.ContainsKey(neighbor))
                {
                    visited.Add(neighbor);
                    stack.Push(neighbor);
                }
        }

        return component;
    }
}