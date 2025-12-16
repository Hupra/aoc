using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 7)]
public sealed class Day07 : IDay
{
    public string SolvePart1(string input)
    {
        var lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        char[][] charLines = [.. lines.Select(line => line.ToCharArray())];

        var sum = 0;

        for (var i = 0; i < charLines.Length - 1; i++)
        for (var j = 0; j < charLines[i].Length; j++)
            switch (charLines[i][j])
            {
                case 'S':
                    charLines[i + 1][j] = '|';
                    break;
                case '|' when charLines[i + 1][j] == '^':
                {
                    sum++;
                    if (charLines[i + 1][j - 1] == '.')
                        charLines[i + 1][j - 1] = '|';
                    if (charLines[i + 1][j + 1] == '.')
                        charLines[i + 1][j + 1] = '|';
                    break;
                }
                case '|':
                {
                    if (charLines[i + 1][j] == '.')
                        charLines[i + 1][j] = '|';
                    break;
                }
            }

        return sum.ToString();
    }

    public string SolvePart2(string input)
    {
        var lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        char[][] table = [.. lines.Select(line => line.ToCharArray())];

        var memo = new Dictionary<(int, int), long>();
        return F(table, 0, lines[0].IndexOf('S'), memo).ToString();
    }

    private static long F(char[][] table, int i, int j, Dictionary<(int, int), long> memo)
    {
        if (i >= table.Length)
            return 1;

        if (memo.TryGetValue((i, j), out var cached))
            return cached;

        var result = table[i][j] == '^'
            ? F(table, i + 2, j - 1, memo) + F(table, i + 2, j + 1, memo)
            : F(table, i + 2, j, memo);

        memo[(i, j)] = result;
        return result;
    }
}
// char[,] table = new char[lines.Length, lines[0].Length];
// for (int i = 0; i < lines.Length; i++)
//     for (int j = 0; j < lines[0].Length; j++)
//         table[i, j] = lines[i][j];