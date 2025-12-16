using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

[Day(2025, 1)]
public sealed class Day01 : IDay
{
    public string SolvePart1(string input)
    {
        var count = 0;
        var dial = 50;
        var lineNumber = 0;

        foreach (var instruction in input.AsSpan().EnumerateLines())
        {
            var (success, turn, amount) = TryParseInstruction(instruction, ++lineNumber);
            if (!success)
                continue;

            if (turn == 'L')
                amount = -amount;

            dial = (dial + amount) % 100;

            if (dial == 0)
                count++;
        }

        return count.ToString();
    }

    public string SolvePart2(string input)
    {
        var count = 0;
        var dial = 50;
        var lineNumber = 0;

        foreach (var instruction in input.AsSpan().EnumerateLines())
        {
            var (success, turn, amount) = TryParseInstruction(instruction, ++lineNumber);
            if (!success)
                continue;

            // Count full rotations
            count += amount / 100;
            amount %= 100;

            if (turn == 'L')
                amount = -amount;

            var newDial = (dial + amount + 100) % 100;

            // Double count prevention for dial=0 case
            // If dial=0, all its rotations are caught by (amount / 100) above
            if (dial != 0 && (turn == 'L' && newDial > dial ||
                              turn == 'R' && newDial < dial ||
                              newDial == 0))
            {
                count++;
            }

            dial = newDial;
        }

        return count.ToString();
    }

    private static (bool Success, char Turn, int Amount) TryParseInstruction(ReadOnlySpan<char> instruction,
        int lineNumber)
    {
        if (instruction.IsEmpty)
        {
            Console.WriteLine($"Empty instruction line at line {lineNumber}");
            return (false, '\0', 0);
        }

        var turn = instruction[0];
        if (!int.TryParse(instruction[1..], out var amount))
        {
            Console.WriteLine($"Failed to parse instruction on line {lineNumber}: {instruction.ToString()}");
            return (false, '\0', 0);
        }

        if (turn is 'L' or 'R') return (true, turn, amount);

        Console.WriteLine($"Invalid turn direction on line {lineNumber}: {instruction.ToString()}");
        return (false, '\0', 0);
    }
}
