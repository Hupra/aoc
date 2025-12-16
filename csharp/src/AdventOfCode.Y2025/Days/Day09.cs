using AdventOfCode.Core;

namespace AdventOfCode.Y2025.Days;

/*
 * Should also be possible to shrink the squares by 0.4 units on each side
 * and then we should be able to say that no edge should intersect the square
 */
[Day(2025, 9)]
public sealed class Day09 : IDay
{
    public string SolvePart1(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1]))
            .ToList();

        var squares = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (size: Area(pair.p1, pair.p2), pair.p1, pair.p2));

        return squares
            .MaxBy(x => x.size)
            .size
            .ToString();
    }

    public string SolvePart2Luck(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1]))
            .ToList();

        var squares = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (area: Area(pair.p1, pair.p2), pair.p1, pair.p2))
            .OrderByDescending(x => x.area);

        points.Add(points.First()); // to form closed loop

        var edgeMidpoints = points
            .Zip(points.Skip(1))
            .Select(p =>
            {
                var midx = (p.First.Item1 + p.Second.Item1) / 2.0;
                var midy = (p.First.Item2 + p.Second.Item2) / 2.0;
                return (midx, midy);
            })
            .ToList();

        var result = squares
            .Where(s => !points.Any(p => IsPointInSquare(p, s.p1, s.p2)))
            .Where(s => !edgeMidpoints.Any(mid => IsPointInSquare(mid, s.p1, s.p2)))
            .FirstOrDefault();

        return result.area.ToString();
    }

    public string SolvePart2RealCheck(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1]))
            .ToList();

        var squares = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (area: Area(pair.p1, pair.p2), pair.p1, pair.p2))
            .OrderByDescending(x => x.area);

        points.Add(points.First()); // to form closed loop

        var edges = points
            .Zip(points.Skip(1))
            .ToList();

        var result = squares
            .Where(s => !points.Any(p => IsPointInSquare(p, s.p1, s.p2)))
            .Where(s => !edges.Any(edge =>
            {
                // check that edge doesnt go THROUGH square
                // must be inside square bounds on one axis,
                // and span full square on other axis
                // it must span because we already knoww if an endpoint
                // is inside the square because it was previously checked above

                var left = Math.Min(s.p1.Item1, s.p2.Item1);
                var bot = Math.Min(s.p1.Item2, s.p2.Item2);
                var right = Math.Max(s.p1.Item1, s.p2.Item1);
                var top = Math.Max(s.p1.Item2, s.p2.Item2);

                // horizontal edge
                if (edge.First.Item2 == edge.Second.Item2) // same Y
                {
                    var edgeMid = edge.First.Item2;
                    var edgeLeft = Math.Min(edge.First.Item1, edge.Second.Item1);
                    var edgeRight = Math.Max(edge.First.Item1, edge.Second.Item1);

                    return bot < edgeMid && edgeMid < top && // Must be strictly inside
                           edgeLeft <= left && right <= edgeRight; // Edge must span full square width
                }
                // vertical edge
                else
                {
                    var edgeMid = edge.First.Item1;
                    var edgeBot = Math.Min(edge.First.Item2, edge.Second.Item2);
                    var edgeTop = Math.Max(edge.First.Item2, edge.Second.Item2);

                    return left < edgeMid && edgeMid < right &&
                           edgeBot <= bot && top <= edgeTop;
                }
            }))
            .FirstOrDefault();


        return result.area.ToString();
    }

    public string SolvePart2(string input)
    {
        var points = input
            .Split('\n', StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(',').Select(int.Parse).ToArray())
            .Select(a => (a[0], a[1]))
            .ToList();

        var squares = points
            .SelectMany((_, i) => points.Skip(i + 1), (p1, p2) => (p1, p2))
            .Select(pair => (area: Area(pair.p1, pair.p2), edges: GetSquareEdges(pair.p1, pair.p2)))
            .OrderByDescending(x => x.area);

        points.Add(points.First()); // to form closed loop

        var edges = points
            .Zip(points.Skip(1))
            .Select(p => (p.First, p.Second))
            .ToList();

        var result = squares
            .Where(square => !square.edges
                .Any(edge1 => edges.Any(edge2 => DoLinesIntersect(
                    edge1.start,
                    edge1.end,
                    edge2.First,
                    edge2.Second
                ))))
            .FirstOrDefault();
        return result.area.ToString();
    }

    private static bool IsPointInSquare(
        (double x, double y) point,
        (int x, int y) corner1,
        (int x, int y) corner2)
    {
        var minx = Math.Min(corner1.x, corner2.x);
        var miny = Math.Min(corner1.y, corner2.y);
        var maxx = Math.Max(corner1.x, corner2.x);
        var maxy = Math.Max(corner1.y, corner2.y);

        return point.x > minx &&
               point.x < maxx &&
               point.y > miny &&
               point.y < maxy;
    }

    private static long Area((long x, long y) p1, (long x, long y) p2)
    {
        return (Math.Abs(p1.x - p2.x) + 1) * (Math.Abs(p1.y - p2.y) + 1);
    }

    private static bool DoLinesIntersect(
        (double x, double y) line1Start,
        (double x, double y) line1End,
        (double x, double y) line2Start,
        (double x, double y) line2End)
    {
        var d1 = CrossProduct(line2Start, line2End, line1Start);
        var d2 = CrossProduct(line2Start, line2End, line1End);
        var d3 = CrossProduct(line1Start, line1End, line2Start);
        var d4 = CrossProduct(line1Start, line1End, line2End);

        // Lines intersect if the endpoints of each line are on opposite sides (or on) the other line
        return ((d1 >= 0 && d2 <= 0) || (d1 <= 0 && d2 >= 0)) &&
               ((d3 >= 0 && d4 <= 0) || (d3 <= 0 && d4 >= 0));

        // Calculate the direction of cross products
        double CrossProduct((double x, double y) o, (double x, double y) a, (double x, double y) b)
        {
            return (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
        }
    }

    private static List<((double x, double y) start, (double x, double y) end)> GetSquareEdges(
        (int x, int y) corner1,
        (int x, int y) corner2)
    {
        // Shrink the square too.
        var left = Math.Min(corner1.x, corner2.x) + 0.1;
        var right = Math.Max(corner1.x, corner2.x) - 0.1;
        var bottom = Math.Min(corner1.y, corner2.y) + 0.1;
        var top = Math.Max(corner1.y, corner2.y) - 0.1;

        return
        [
            ((left, bottom), (right, bottom)), // Bottom edge
            ((right, bottom), (right, top)), // Right edge
            ((right, top), (left, top)), // Top edge
            ((left, top), (left, bottom))
        ];
    }
}
