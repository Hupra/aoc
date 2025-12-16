using System.Globalization;
using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 3)]
public sealed class Day03 : IDay
{
    public string SolvePart1(string input)
    {
        var res = 0;
        foreach (var line in input.AsSpan().EnumerateLines())
        {
            var bank = line
                .ToArray()
                .Select(c => c - '0')
                .ToArray();

            var first = bank
                .SkipLast(1)
                .Select((v, i) => (v, i))
                .MaxBy(x => x.v);

            var second = bank
                .Skip(first.i + 1)
                .Select((v, i) => (v, i))
                .MaxBy(x => x.v);

            res += first.v * 10 + second.v;
        }

        return res.ToString();
    }

    public string SolvePart2(string input)
    {
        double res = 0;
        foreach (var line in input.AsSpan().EnumerateLines())
        {
            int[] bank = [.. line.ToArray().Select(c => c - '0')];
            var skip = 0;
            double bankRes = 0;

            for (var i = 11; i >= 0; i--)
            {
                var digit = bank
                    .Skip(skip)
                    .SkipLast(i)
                    .Select((v, idx) => (v, idx))
                    .MaxBy(x => x.v);
                skip += digit.idx + 1;
                bankRes += Math.Pow(10, i) * digit.v;
            }

            res += bankRes;
        }

        return res.ToString(CultureInfo.InvariantCulture);
    }
}