namespace AdventOfCode.Core.Grid;

public readonly record struct Point(int I, int J)
{
    public bool InBounds<T>(T[,] grid) => I >= 0 && I < grid.GetLength(0) && J >= 0 && J < grid.GetLength(1);

    public IEnumerable<Point> Neighbors4()
    {
        return
        [
            this with { J = J - 1 },
            this with { I = I + 1 },
            this with { J = J + 1 },
            this with { I = I - 1 }
        ];
    }

    public IEnumerable<Point> Neighbors4<T>(T[,] grid) => Neighbors4().Where(p => p.InBounds(grid));

    public IEnumerable<Point> Neighbors8()
    {
        for (var di = -1; di <= 1; di++)
            for (var dj = -1; dj <= 1; dj++)
                if (!(di == 0 && dj == 0))
                    yield return new Point(I + di, J + dj);
    }

    public IEnumerable<Point> Neighbors8<T>(T[,] grid) => Neighbors8().Where(p => p.InBounds(grid));
}