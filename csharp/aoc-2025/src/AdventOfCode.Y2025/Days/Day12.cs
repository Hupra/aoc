using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 12)]
public sealed class Day12 : IDay
{
    public string SolvePart1(string input)
    {
        var (pieces, puzzles) = ParseInput(input);

        var works = 0;
        foreach (var (grid, todo) in puzzles)
        {
            if (grid.GetLength(0) * grid.GetLength(1) >= todo.Sum() * 9)
            {
                works++;
            }
        }

        return works.ToString();
    }

    public string SolvePart2(string input)
    {
        return "There is no part 2";
    }

    private static (List<int[,]> pieces, List<(int[,] grid, int[] todo)> puzzles) ParseInput(string input)
    {
        var lines = input.Split("\n\n", StringSplitOptions.RemoveEmptyEntries);

        var pieces = lines
            .SkipLast(1)
            .Select(line => line.Split('\n'))
            .Select(arr => arr.Skip(1).ToList())
            .Select(grid =>
            {
                var piece = new int[3, 3];
                for (var y = 0; y < grid.Count; y++)
                for (var x = 0; x < grid[y].Length; x++)
                    if (grid[y][x] == '#')
                        piece[x, y] = 1;
                return piece;
            })
            .ToList();

        var puzzles = lines
            .Last()
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line =>
                {
                    var parts = line
                        .Split(' ')
                        .ToList();
                    var xy = parts.First()[..^1].Split('x').Select(int.Parse).ToArray();
                    var gird = new int[xy[0], xy[1]];
                    var todo = parts
                        .Skip(1)
                        .Select(int.Parse)
                        .ToArray();
                    return (gird, todo);
                }
            )
            .ToList();


        return (pieces, puzzles);
    }
}
