sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.trim.lines>>.comb>>.List;

    my %regions;
    my %parent-regions;
    for ^@map.elems X ^@map[0].elems -> $pos {
        next if %regions{"{$pos[0]};{$pos[1]}"}:exists;
        grow-region(@map, %regions, $pos, $pos.Str);
    }
    my %rev-regions;
    for %regions {
        %rev-regions{.value} = [] if %rev-regions{.value}:!exists;
        %rev-regions{.value}.push(.key.split(";")>>.Int.List);
    }

    my $part1-solution = 0;
    for %rev-regions {
        my $plant = @map[.value[0][0]][.value[0][1]];
        my $area = .value.elems;
        my $perimeter = 0;
        for .value.Seq {
            my @to-check = ($_[0] - 1, $_[1]), ($_[0], $_[1] - 1), ($_[0] + 1, $_[1]), ($_[0], $_[1] + 1);
            for @to-check -> ($check-row, $check-col) {
                if $check-row < 0 or $check-col < 0 or $check-row >= @map.elems or $check-col >= @map[0].elems or @map[$check-row][$check-col] ne $plant {
                    $perimeter++;
                }
            }
        }
        $part1-solution += $area * $perimeter;
    }

    say "part 1: $part1-solution";

    my $part2-solution = 0;
    my @sides = (0 xx @map[0].elems).Array xx @map.elems;
    for %rev-regions {
        my $plant = @map[.value[0][0]][.value[0][1]];
        my $area = .value.elems;
        my $sides = 0;
        for .value.Seq -> ($row, $col) {
            # up, left, down, right
            my @to-check = ($row - 1, $col), ($row, $col - 1), ($row + 1, $col), ($row, $col + 1);
            my @side-label = (1, 2, 4, 8);
            my $this-side = 0;
            for ^4 {
                my ($check-row, $check-col) = @to-check[$_]; 
                if $check-row < 0 or $check-col < 0 or $check-row >= @map.elems or $check-col >= @map[0].elems or @map[$check-row][$check-col] ne $plant {
                    $this-side +|= @side-label[$_];
                }
            }
            @sides[$row][$col] = $this-side;
        }
        for .value.Seq -> ($row, $col) {
            my $this-side = @sides[$row][$col];
            if $this-side +& 1 {
                my $left-cell-up = ($col > 0) && @sides[$row][$col-1] +& 1 && ((@map[$row][$col-1]) eq $plant);
                $sides++ if not ($left-cell-up);
            }
            if $this-side +& 4 {
                my $left-cell-down = $col > 0 && (@sides[$row][$col-1] +& 4) && @map[$row][$col-1] eq $plant;
                $sides++ if not ($left-cell-down);
            }
            if $this-side +& 2 {
                my $up-cell-left = $row > 0 && (@sides[$row-1][$col] +& 2) && @map[$row-1][$col] eq $plant;
                $sides++ if not ($up-cell-left);
            }
            if $this-side +& 8 {
                my $up-cell-right = $row > 0 && (@sides[$row-1][$col] +& 8) && @map[$row-1][$col] eq $plant;
                $sides++ if not ($up-cell-right);
            }
        }
        $part2-solution += $area * $sides;
    }
    say "part 2: $part2-solution";
}

sub grow-region(@map, %regions, ($row, $col), $region) {
    %regions{"$row;$col"} = $region;
    my @to-check = ($row - 1, $col), ($row, $col - 1), ($row + 1, $col), ($row, $col + 1);
    for @to-check -> ($check-row, $check-col) {
        next if $check-row < 0 or $check-col < 0;
        next if $check-row >= @map.elems or $check-col >= @map[0].elems;
        next if @map[$row][$col] ne @map[$check-row][$check-col];
        next if %regions{"$check-row;$check-col"}:exists;
        %regions{"$check-row;$check-col"} = $region;
        grow-region(@map, %regions, ($check-row, $check-col), $region);
    }
}