using AdventOfCode.Core;
using AdventOfCode.Core.Utilities;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 8)]
public sealed class Day08 : IDay
{
    private static double Dist((int x, int y, int z) p1, (int x, int y, int z) p2)
    {
        return Math.Sqrt(
            Math.Pow(p1.x - p2.x, 2) +
            Math.Pow(p1.y - p2.y, 2) +
            Math.Pow(p1.z - p2.z, 2)
        );
    }

    public string SolvePart1(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1], a[2]))
            .ToList();

        var pointPairs = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (dist: Dist(pair.p1, pair.p2), pair.p1, pair.p2))
            .OrderBy(x => x.dist);

        var uf = points
            .ToDictionary(p => p, p => p);

        pointPairs
            .Take(points.Count < 1000 ? 10 : 1000)
            .ToList()
            .ForEach(edge => Union(edge.p1, edge.p2, uf));

        var rootsSize = uf.Values
            .Select(p => FindRoot(p, uf))
            .GroupBy(r => r)
            .Select(g => g.Count())
            .OrderByDescending(c => c)
            .Take(3)
            .ToList();

        return rootsSize
            .Aggregate(1, (acc, size) => acc * size)
            .ToString();
    }


    public string SolvePart2(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1], a[2]))
            .ToList();

        var pointPairs = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (dist: Dist(pair.p1, pair.p2), pair.p1, pair.p2))
            .OrderBy(x => x.dist)
            .ToList();

        var uf = points
            .ToDictionary(p => p, p => p);

        var componentCount = points.Count;

        var takenCount = pointPairs
            .TakeWhile(_ => componentCount > 1)
            .Select(edge =>
            {
                if (Union(edge.p1, edge.p2, uf)) componentCount--;
                return edge;
            })
            .Count();

        var pair = pointPairs[takenCount - 1];
        return ((long)pair.p1.Item1 * pair.p2.Item1).ToString();
    }

    private static T FindRoot<T>(T p, Dictionary<T, T> uf) where T : notnull
    {
        if (!uf[p].Equals(p))
            uf[p] = FindRoot(uf[p], uf);
        return uf[p];
    }

    private static bool Union<T>(T p1, T p2, Dictionary<T, T> uf) where T : notnull
    {
        var root1 = FindRoot(p1, uf);
        var root2 = FindRoot(p2, uf);
        if (root1.Equals(root2))
            return false;
        uf[root1] = root2;
        return true;
    }


/////////////////////////////////////////////

    public string SolvePart1Graph(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1], a[2]))
            .ToList();

        var pointPairs = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (dist: Dist(pair.p1, pair.p2), pair.p1, pair.p2))
            .OrderBy(x => x.dist);

        var adj = points
            .ToDictionary(p => p, _ => new List<(int x, int y, int z)>());

        pointPairs
            .Take(points.Count < 1000 ? 10 : 1000)
            .ToList()
            .ForEach(edge =>
            {
                adj[edge.p1].Add(edge.p2);
                adj[edge.p2].Add(edge.p1);
            });

        return GraphUtilities
            .GetAllConnectedComponents(adj)
            .OrderByDescending(c => c.Count)
            .Take(3)
            .Aggregate(1, (acc, comp) => acc * comp.Count)
            .ToString();
    }

    public string SolvePart2Graph(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1], a[2]))
            .ToList();

        var pointPairs = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (dist: Dist(pair.p1, pair.p2), pair.p1, pair.p2))
            .OrderBy(x => x.dist)
            .ToList();

        var result = SearchUtilities.BinarySearchFirst(
            0,
            pointPairs.Count,
            mid => IsFullyConnected(points, pointPairs.Take(mid))
        );

        var pair = pointPairs[result - 1];
        return ((long)pair.p1.Item1 * pair.p2.Item1).ToString();
    }

    private static bool IsFullyConnected(
        List<(int x, int y, int z)> points,
        IEnumerable<(double dist, (int x, int y, int z) p1, (int x, int y, int z) p2)> edges)
    {
        var adj = points.ToDictionary(p => p, _ => new List<(int x, int y, int z)>());

        foreach (var (_, p1, p2) in edges)
        {
            adj[p1].Add(p2);
            adj[p2].Add(p1);
        }

        return GraphUtilities.GetAllConnectedComponents(adj).Count == 1;
    }
}