using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

/*
 * Should also be possible to shrink the squares by 0.4 units on each side
 * and then we should be able to say that no edge should intersect the square
 */
[Day(2025, 11)]
public sealed class Day11 : IDay
{
    public string SolvePart1(string input)
    {
        var lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var adj = new Dictionary<string, List<string>>();
        foreach (var line in lines)
        {
            var parts = line.Split(' ');
            var s = parts[0][..^1];
            adj.Add(s, parts.Skip(1).ToList());
        }

        adj.TryAdd("out", new List<string>());
        adj.TryAdd("you", new List<string>());

        return DFS("you", "out", adj, new HashSet<string>()).ToString();
    }


    public string SolvePart2(string input)
    {
        var lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var adj = new Dictionary<string, List<string>>();
        foreach (var line in lines)
        {
            var parts = line.Split(' ');
            var s = parts[0][..^1];
            adj.Add(s, parts.Skip(1).ToList());
        }

        adj.TryAdd("out", new List<string>());
        adj.TryAdd("you", new List<string>());


        return DFS2("svr", "out", adj, new HashSet<string>(), new Dictionary<(string, bool, bool), long>()).ToString();
    }


    private static int DFS(string s, string t, Dictionary<string, List<string>> adj, HashSet<string> visited)
    {
        if (s == t)
            return 1;

        visited.Add(s);

        var result = 0;
        foreach (var b in adj[s])
        {
            if (!visited.Contains(b))
            {
                result += DFS(b, t, adj, visited);
            }
        }

        visited.Remove(s);
        return result;
    }

    private static long DFS2(string s, string t, Dictionary<string, List<string>> adj,
        HashSet<string> visited, Dictionary<(string, bool, bool), long> memo)
    {
        var hasDac = visited.Contains("dac");
        var hasFft = visited.Contains("fft");

        if (s == t)
            return (hasDac && hasFft) ? 1 : 0;

        var key = (s, hasDac, hasFft);
        if (memo.TryGetValue(key, out var cached))
            return cached;

        visited.Add(s);

        var result = 0L;
        foreach (var b in adj[s])
        {
            if (!visited.Contains(b))
            {
                result += DFS2(b, t, adj, visited, memo);
            }
        }

        visited.Remove(s);

        memo[key] = result;
        return result;
    }
}
