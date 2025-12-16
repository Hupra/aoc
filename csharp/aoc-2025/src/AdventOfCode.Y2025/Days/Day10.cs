using AdventOfCode.Core;
using Microsoft.Z3;

namespace AdventOfCode.Y2025.Days;

/*
 * Should also be possible to shrink the squares by 0.4 units on each side
 * and then we should be able to say that no edge should intersect the square
 */
[Day(2025, 10)]
public sealed class Day10 : IDay
{
    private static string ToBinaryString(int value, int bits = 8)
    {
        return Convert.ToString(value, 2).PadLeft(bits, '0');
    }

    private static int FlipBits(int state, int flip)
    {
        return state ^ flip;
    }

    public string SolvePart1(string input)
    {
        var games = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(' ').ToList())
            .Select(game =>
            {
                var goal = game
                    .First()[1..^1]
                    .ToCharArray()
                    .Select((c, i) => (c, i))
                    .Where(t => t.c == '#')
                    .Select(t => 1 << t.i)
                    .Aggregate(0, (acc, b) => acc | b);

                var buttons = game
                    .Skip(1)
                    .SkipLast(1)
                    .Select(s => s[1..^1]
                        .Split(',')
                        .Select(n => 1 << (int.Parse(n)))
                        .Aggregate(0, (acc, b) => acc | b))
                    .ToList();

                var queue = new Queue<(int state, int depth)>();
                var visited = new HashSet<int>();
                queue.Enqueue((0, 0));

                while (queue.Count > 0)
                {
                    var (state, depth) = queue.Dequeue();

                    if (!visited.Add(state))
                        continue;

                    if (state == goal)
                        return depth;

                    foreach (var newState in buttons.Select(b => FlipBits(state, b)))
                        queue.Enqueue((newState, depth + 1));
                }

                return int.MaxValue;
            })
            .Sum();


        return games.ToString();
    }

    public string SolvePart2(string input)
    {
        var games = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(' ').ToList())
            .Select(game =>
            {
                var goal = game
                    .Last()[1..^1]
                    .Split(',')
                    .Select(int.Parse)
                    .ToArray();

                var buttonsGroups = game
                    .Skip(1)
                    .SkipLast(1)
                    .Select(s => s[1..^1]
                        .Split(',')
                        .Select(int.Parse)
                        .ToArray())
                    .ToList();

                // return SolveWithDP(goal, buttonsGroups, new Dictionary<string, int>());
                return SolveWithZ3(goal, buttonsGroups);
            })
            .Sum();

        return games.ToString();
    }

    private static int SolveWithZ3(int[] goal, List<int[]> buttonsGroups)
    {
        using var ctx = new Context();
        using var solver = ctx.MkOptimize();

        // Create an integer variable for each button representing how many times it's pressed
        // MkIntConst creates a symbolic integer constant (variable) with the given name
        var buttonVars = buttonsGroups
            .Select(buttons => string.Join(",", buttons))
            .ToDictionary(key => key, key => ctx.MkIntConst(key));

        // Add constraint: each button must be pressed >= 0 times (non-negative)
        // MkGe creates a "greater than or equal" comparison expression
        // MkInt creates an integer literal (constant value 0)
        foreach (var buttonVar in buttonVars.Values)
            solver.Add(ctx.MkGe(buttonVar, ctx.MkInt(0)));

        // 2,4,3,1,3
        // add contraints for each position in the goal
        for (var pos = 0; pos < goal.Length; pos++)
        {
            // Collect all button variables that affect this position
            var terms = buttonsGroups
                .Where(buttons => buttons.Contains(pos))
                .Select(buttons => string.Join(",", buttons))
                .Select(buttons => buttonVars[buttons])
                .Select(v => ctx.MkMul(ctx.MkInt(1), v))
                .ToList();

            switch (terms.Count)
            {
                // the sum of all button presses that affect this position must equal goal[pos]
                case > 0:
                    solver.Add(ctx.MkEq(ctx.MkAdd(terms), ctx.MkInt(goal[pos])));
                    break;
                default:
                    solver.Add(ctx.MkEq(ctx.MkInt(0), ctx.MkInt(goal[pos])));
                    break;
            }
        }

        // MkAdd sums all button press counts to get total presses
        var totalPresses = ctx.MkAdd(buttonVars.Values);

        // Minimize the total number of button presses
        solver.MkMinimize(totalPresses);

        if (solver.Check() != Status.SATISFIABLE) return int.MaxValue / 2;

        var model = solver.Model;
        return buttonVars.Values.Sum(expr => ((IntNum)model.Evaluate(expr)).Int);
    }

    private static int SolveWithDP(int[] state, List<int[]> buttonsGroups, Dictionary<string, int> memo)
    {
        var stateKey = string.Join(",", state);
        // Console.WriteLine($"{memo.Count}");
        // Console.Write($"Evaluating state [{string.Join(",", state)}]... ");
        // Thread.Sleep(100);
        if (memo.TryGetValue(stateKey, out var cached))
        {
            // Console.Write($"Cache hit for state [{string.Join(",", state)}]: {cached}\n");
            return cached;
        }

        if (state.Any(x => x < 0))
        {
            return int.MaxValue / 2;
        }

        if (state.Sum() == 0)
        {
            Console.WriteLine("Reached goal state!");
            const int r = 0;
            memo[stateKey] = r;
            return r;
        }

        var res = buttonsGroups
            .Select(buttons =>
            {
                var newState = state.ToArray();
                foreach (var b in buttons)
                    newState[b]--;

                return SolveWithDP(newState, buttonsGroups, memo) + 1;
            })
            .Min();
        memo[stateKey] = res;
        return res;
    }
}


// 0 = 00000001
// 1 = 00000010
// 2 = 00000100
// 3 = 00001000
// 4 = 00010000