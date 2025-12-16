using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 5)]
public sealed class Day05 : IDay
{
    public string SolvePart1(string input)
    {
        var (ranges, numbers) = ParseInput(input);
        return numbers
            .Where(number => ranges.Any(range => range.Contains(number)))
            .Count()
            .ToString();
    }

    public string SolvePart2(string input)
    {
        var ranges = ParseInput(input).ranges
            .OrderBy(r => r.Start)
            .ToList();

        if (ranges.Count == 0) return "0";

        var merged = new List<Range>();
        var expanding = ranges[0];

        foreach (var inspecting in ranges)
        {
            if (expanding.FastOverlaps(inspecting))
            {
                expanding = expanding.Merge(inspecting);
            }
            else
            {
                merged.Add(expanding);
                expanding = inspecting;
            }
        }

        merged.Add(expanding);

        return merged.Sum(r => r.End - r.Start + 1).ToString();
    }

    private static (IEnumerable<Range> ranges, IEnumerable<long> numbers) ParseInput(string input)
    {
        var parts = input.Split("\n\n");
        var ranges = parts[0]
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(part =>
            {
                var nums = part.Split('-').Select(long.Parse).ToArray();
                return new Range(nums[0], nums[1]);
            });

        var numbers = parts[1]
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(long.Parse);

        return (ranges, numbers);
    }

    private class Range(long start, long end)
    {
        public long Start { get; } = start;
        public long End { get; } = end;

        public bool Contains(long number) => number >= Start && number <= End;

        // public bool Overlaps(Range other) => Start <= other.End && other.Start <= End;

        // only works when for the sorted case where we know Start <= other.End
        public bool FastOverlaps(Range other) => other.Start <= End;

        // nice logic when we care about finding the exact overlap
        // public bool OverlapsMax(Range other) => Math.Max(Start, other.Start) <= Math.Min(End, other.End);

        public Range Merge(Range other) => new(Math.Min(Start, other.Start), Math.Max(End, other.End));

        public override string ToString() => $"{Start}-{End}";
    }
}