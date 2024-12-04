sub MAIN($input) {
    my $file = (open $input).slurp;
    my @grid is List = $file.lines».comb».list;
    my @transposedGrid is List = [Z] @grid;
    my @reversedGrid is List = @grid».reverse;
    my @transposedReversedGrid is List = @transposedGrid».reverse;

    my @horizontalScanRows is List = generateScanHorizontal(@grid);
    my @transposedHorizontalScanRows is List = generateScanHorizontal(@transposedGrid);

    my @part-one-counts = [];
    @part-one-counts.push(count-xmas(@grid, @horizontalScanRows)); # Right
    @part-one-counts.push(count-xmas(@transposedGrid, @transposedHorizontalScanRows)); # Down
    @part-one-counts.push(count-xmas(@reversedGrid, @horizontalScanRows)); # Left
    @part-one-counts.push(count-xmas(@transposedReversedGrid, @transposedHorizontalScanRows)); # Up

    my @diagonalScanRows is List = generateScanDiagonal(@grid);
    my @transposedDiagonalScanRows is List = generateScanDiagonal(@transposedGrid);
    @part-one-counts.push(count-xmas(@grid, @diagonalScanRows)); # Down Right
    @part-one-counts.push(count-xmas(@grid, @diagonalScanRows».reverse)); # Up Left
    @part-one-counts.push(count-xmas(@reversedGrid, @diagonalScanRows)); # Down Left
    @part-one-counts.push(count-xmas(@reversedGrid, @diagonalScanRows».reverse)); # Up Right

    my $part-one-solution = @part-one-counts.sum;
    say "part 1: $part-one-solution";


    my @part-two-counts = [];
    @part-two-counts.push(countGridMatches(@grid, (<M . S>,<. A .>,<M . S>)));
    @part-two-counts.push(countGridMatches(@grid, (<S . S>,<. A .>,<M . M>)));
    @part-two-counts.push(countGridMatches(@grid, (<S . M>,<. A .>,<S . M>)));
    @part-two-counts.push(countGridMatches(@grid, (<M . M>,<. A .>,<S . S>)));

    my $part-two-solution = @part-two-counts.sum;
    say "part 2: $part-two-solution";

}

sub count-xmas(@grid, @scanRows) {
    my $xmas-count = 0;
    for @scanRows -> @scanRow {
        my $xmas-pos = 0;
        for @scanRow -> @pos {
            my $char = @grid[@pos[0]][@pos[1]];
            if "X" eq $char {
                $xmas-pos = 1;
            }elsif <X M A S>[$xmas-pos] eq $char {
                if $xmas-pos == 3 {
                    $xmas-pos = 0;
                    $xmas-count += 1;
                } else {
                    $xmas-pos += 1;
                }
            } else {
                $xmas-pos = 0;
            }
        }
    }
    return $xmas-count;
}

sub generateScanHorizontal(@grid) {
    # Horizontal
    my $rows = @grid.elems;
    my $cols = @grid[0].elems;
    my @scanRows = ();
    for 0..^$rows -> $row {
        my @scanRow = ();
        for 0..^$cols -> $col {
            @scanRow.push(($row, $col));
        }
        @scanRows.push(@scanRow);
    }
    return @scanRows.List».List;
}

sub generateScanDiagonal(@grid) {
    # Down-right diagonal
    my $rows = @grid.elems;
    my $cols = @grid[0].elems;
    my @scanRows = ();
    for 0..^($rows + $cols - 1) -> $diag {
        my @scanRow = ();
        my $starting-row = max(-$cols + $diag + 1, 0);
        my $starting-col = max($rows - $diag - 1, 0);
        my $diag-len = min($rows - $starting-row, $cols - $starting-col);
        for 0..^$diag-len -> $diag-pos {
            @scanRow.push(($starting-row + $diag-pos, $starting-col + $diag-pos));
        }
        @scanRows.push(@scanRow);
    }
    return @scanRows.List».List;
}

sub countGridMatches(@grid, @needle) {
    my $count = 0;
    for 0..(@grid.elems - @needle.elems) -> $top {
        TOP-LEFT:
        for 0..(@grid[$top].elems - @needle[0].elems) -> $left {
            for 0..^@needle.elems -> $row-offset {
                for 0..^@needle[$row-offset].elems -> $col-offset {
                    my $needle-char = @needle[$row-offset][$col-offset];
                    next if $needle-char eq ".";
                    next TOP-LEFT if $needle-char ne @grid[$top+$row-offset][$left+$col-offset];
                }
            }
            $count += 1;
        }
    }
    return $count;
}