sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.trim.lines>>.comb>>.List;
    my @start-pos = find-pos(@map, "S");
    my @end-pos = find-pos(@map, "E");

    my %step-lookup;
    my $steps = 0;
    my @to-search = ((@start-pos.List),);
    while @to-search.elems {
        my @p = @to-search.shift.List;
        next if @map[@p[0]][@p[1]] eq "#";
        next if %step-lookup{index-p(@p)}:exists;
        %step-lookup{index-p(@p)} = $steps++;
        @to-search.append(
            (@p Z+ ( 0,  1)),
            (@p Z+ ( 0, -1)),
            (@p Z+ ( 1,  0)),
            (@p Z+ (-1,  0)),
        );
    }

    my $shortcut-savings = BagHash.new();
    for %step-lookup.kv -> $p, $base-steps {
        my @p = $p.split(";")>>.Int;
        for radius-points(2) -> @jump {
            my @np = @p Z+ @jump;
            next if %step-lookup{index-p(@np)}:!exists;
            my $time-saved = %step-lookup{index-p(@np)} - $base-steps - 2;
            $shortcut-savings{$time-saved}++ if $time-saved > 0;
        }
    }

    my $part1-solution = $shortcut-savings.grep(*.key >= 100).map(*.value).sum;
    say "part1 solution: $part1-solution";

    my $part2-shortcut-savings = BagHash.new();
    for %step-lookup.kv -> $p, $base-steps {
        my @p = $p.split(";")>>.Int;
        for 2..20 -> $radius {
            for radius-points($radius) -> @jump {
                my @np = @p Z+ @jump;
                next if %step-lookup{index-p(@np)}:!exists;
                my $time-saved = %step-lookup{index-p(@np)} - $base-steps - $radius;
                $part2-shortcut-savings{$time-saved}++ if $time-saved > 0;
            }
        }
    }

    my $part2-solution = $part2-shortcut-savings.grep(*.key >= 100).map(*.value).sum;
    say "part2 solution: $part2-solution";
}

sub index-p(@p) {
    "{@p[0]};{@p[1]}"
}

sub find-pos(@map, $needle) {
    for ^@map.elems X ^@map[0].elems -> ($row, $col) {
        return ($row, $col) if @map[$row][$col] eq $needle;
    }
    return Nil;
}

sub radius-points($radius) {
    (0...$radius...0...-$radius...-1) Z, ($radius...0...-$radius...0...($radius-1))
}