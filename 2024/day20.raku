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
    my %soup;
    my @right-offsets = radius-points(20).grep(*.[1] >= 0 && *.[0] != 20);
    for (0..@map.elems) X (-20..@map[0].elems) -> @p {
        %soup = Hash.new() if @p[1] == -20;
        %soup{index-p(@p Z+ (0, 19))} = %step-lookup{index-p(@p Z+ (0, 19))} if %step-lookup{index-p(@p Z+ (0, 19))}:exists;
        %soup{index-p(@p Z+ (0, -1))}:delete;
        
        my @new-compares = @right-offsets.map(-> @o {(@o Z+ @p).List}).grep({%step-lookup{index-p($_)}:exists});
        for @new-compares -> @nc {
            my $nc-value = %step-lookup{index-p(@nc)};
            for %soup {
                my @sp = .key.split(";")>>.Int;
                my $distance = (@nc Z- @sp)>>.abs.sum;
                my $time-saved = $nc-value - .value - $distance;
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