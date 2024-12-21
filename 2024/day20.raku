sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.trim.lines>>.comb>>.List;
    my @start-pos = find-pos(@map, "S");
    my @end-pos = find-pos(@map, "E");

    my @step-lookup = [[Nil xx @map[0].elems] xx @map.elems];
    my $steps = 0;
    my @search-p := @start-pos.List;
    search: loop {
        my @p := @search-p;
        @step-lookup[@p[0]][@p[1]] = $steps++;
        for ((@p[0], @p[1]+1),(@p[0], @p[1]-1),(@p[0]+1, @p[1]),(@p[0]-1, @p[1])) -> @np {
            if @map[@np[0]][@np[1]] ne "#" && !@step-lookup[@np[0]][@np[1]].defined {
                @search-p := @np;
                next search;
            }
        }
        last;
    }

    my $shortcut-savings = BagHash.new();
    my @radius2-points := radius-points(2).List;
    for ^@map.elems X ^@map[0].elems -> @p {
        my $base-steps = @step-lookup[@p[0]][@p[1]];
        next unless $base-steps.defined;
        for @radius2-points -> @jump {
            my @np := (@p[0] + @jump[0], @p[1] + @jump[1]);
            my $np-steps = @step-lookup[@np[0]][@np[1]];
            next unless $np-steps.defined;
            my $time-saved = $np-steps - $base-steps - 2;
            $shortcut-savings{$time-saved}++ if $time-saved > 0;
        }
    }

    my $part1-solution = $shortcut-savings.grep(*.key >= 100).map(*.value).sum;
    say "part1 solution: $part1-solution";

    ## Community part2 solution
    # my $part2-solution = 0;
    # my @step-lookup = %step-lookup.map({.key.split(";")>>.Int.List => .value}).sort({.value});
    # for ^@step-lookup.elems -> $i {
    #     for $i+102..^@step-lookup.elems -> $j {
    #         ## That is, until making this change
    #         # my $distance = (@step-lookup[$j].key Z- @step-lookup[$i].key)>>.abs.sum;
    #         my $distance = (@step-lookup[$j].key[0] - @step-lookup[$i].key[0]).abs + (@step-lookup[$j].key[1] - @step-lookup[$i].key[1]).abs;
    #         next unless $distance <= 20;
    #         my $time-saved = @step-lookup[$j].value - @step-lookup[$i].value - $distance;
    #         $part2-solution++ if $time-saved >= 100;
    #     }
    # }

    my $part2-shortcut-savings = BagHash.new();
    my @area20;
    for 2..20 -> $radius {
        for radius-points($radius) -> @p {
            @area20.push((@p, $radius)) if @p[0] > 0 || @p[0] == 0 && @p[1] > 0;
        }
    }
    for ^@map.elems X ^@map[0].elems -> @p {
        my $base-steps = @step-lookup[@p[0]][@p[1]];
        next unless $base-steps.defined;
        for @area20 -> (@jump, $distance) {
            my @np := (@p[0] + @jump[0], @p[1] + @jump[1]);
            next unless 0 <= @np[0] < @map.elems;
            next unless 0 <= @np[1] < @map[0].elems;
            my $np-steps = @step-lookup[@np[0]][@np[1]];
            next unless $np-steps.defined;
            my $time-saved = abs($np-steps - $base-steps) - $distance;
            $part2-shortcut-savings{$time-saved}++ if $time-saved > 0;
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