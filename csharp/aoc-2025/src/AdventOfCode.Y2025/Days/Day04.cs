using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 4)]
public sealed class Day04 : IDay
{
    // 1 -- paper
    // 0 -- empty
    public string SolvePart1(string input)
    {
        var lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var m = ParseGrid(lines);
        var sum = 0;

        for (var i = 0; i < m.GetLength(0); i++)
        for (var j = 0; j < m.GetLength(1); j++)
            if (m[i, j] == 1)
                if (new Core.Grid.Point(i, j).Neighbors8(m).Sum(n => m[n.I, n.J]) < 4)
                    sum++;

        return sum.ToString();
    }

    public string SolvePart2(string input)
    {
        var lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var m = ParseGrid(lines);
        var sum = 0;

        var progressing = true;
        while (progressing)
        {
            progressing = false;
            for (var i = 0; i < m.GetLength(0); i++)
            for (var j = 0; j < m.GetLength(1); j++)
                if (m[i, j] == 1)
                    if (new Core.Grid.Point(i, j).Neighbors8(m).Sum(n => m[n.I, n.J]) < 4)
                    {
                        sum++;
                        m[i, j] = 0;
                        progressing = true;
                    }
        }

        return sum.ToString();
    }

    private static int[,] ParseGrid(string[] lines)
    {
        var height = lines.Length;
        var width = lines[0].Length;
        var grid = new int[height, width];

        for (var i = 0; i < height; i++)
        for (var j = 0; j < width; j++)
            grid[i, j] = lines[i][j] == '@' ? 1 : 0;

        return grid;
    }
}