using AdventOfCode.Core;
using AdventOfCode.Core.Extensions;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 6)]
public sealed class Day06 : IDay
{
    public string SolvePart1(string input)
    {
        List<string[]> rows =
        [
            .. input
                .Split('\n', StringSplitOptions.RemoveEmptyEntries)
                .Select(row => row.Split(' ', StringSplitOptions.RemoveEmptyEntries))
        ];
        var signs = rows.Last();
        rows.RemoveAt(rows.Count - 1);

        return Enumerable.Range(0, signs.Length)
            .Select(i => rows.Select(r => long.Parse(r[i])))
            .Select((nums, i) => signs[i] switch
            {
                "*" => nums.Aggregate((a, b) => a * b),
                _ => nums.Sum()
            })
            .Sum()
            .ToString();
    }

    public string SolvePart2(string input)
    {
        List<string> rows = [.. input.Split('\n', StringSplitOptions.RemoveEmptyEntries)];
        var signs = rows.Last().Split(' ', StringSplitOptions.RemoveEmptyEntries);
        rows.RemoveAt(rows.Count - 1);

        return Enumerable
            .Range(0, rows[0].Length)
            .Select(col => new string([.. rows.Select(row => row[col])])) // transposed
            .Select(s => s.Trim())
            .SplitBy(string.IsNullOrWhiteSpace)
            .Select(nums => nums.Select(long.Parse))
            .Select((nums, i) => signs[i] switch
            {
                "*" => nums.Aggregate((a, b) => a * b),
                _ => nums.Sum()
            })
            .Sum()
            .ToString();
    }

    // public string SolvePart2Old(string input)
    // {
    //     List<string> rows = [.. input.Split('\n', StringSplitOptions.RemoveEmptyEntries)];
    //     var signs = rows.Last().Split(' ', StringSplitOptions.RemoveEmptyEntries);
    //     rows.RemoveAt(rows.Count - 1);

    //     var transposed = Enumerable
    //         .Range(0, rows[0].Length)
    //         .Select(col => new string([.. rows.Select(row => row[col])]))
    //         .Select(s => s.Trim());

    //     return string.Join('\n', transposed)
    //         .Split("\n\n")
    //         .Select(s => 
    //             s.Split('\n', StringSplitOptions.RemoveEmptyEntries)
    //             .Select(long.Parse))
    //         .Select((nums, i) => signs[i] switch
    //         {
    //             "*" => nums.Aggregate((a, b) => a * b),
    //             _   => nums.Sum()
    //         })
    //         .Sum()
    //         .ToString();
    // }
}


