sub MAIN($input) {
    grammar Input {
        token TOP { <byte>+%"\n" "\n"* }
        token byte { (\d+) "," (\d+) }
    }
    my $parsed = Input.parsefile($input);
    my @bytes = $parsed<byte>.map({$_>>.Int.List});

    my $grid-size = @bytes>>.max.max + 1;
    my @map = [['.' xx $grid-size] xx $grid-size];
    simulate(@map, @bytes[^1024]);
    # say @map>>.join().join("\n");
    my $part1-solution = find-path-len(@map);
    say "part1 solution: $part1-solution";

    @map = [['.' xx $grid-size] xx $grid-size];
    my $part2-index = find-closing-byte(@map, @bytes);
    my $part2-solution = @bytes[$part2-index].join(",");
    say "part2-solution: $part2-solution";
}

sub simulate(@map, @bytes) {
    for @bytes {
        @map[.[1]][.[0]] = "#";
    }
}

sub find-path-len(@map) {
    my @to-search = [((0,0), 0),];
    my @scores = [[Nil xx @map[1].elems] xx @map.elems];
    while @to-search.shift -> (@pos, $steps) {
        next unless 0 <= @pos[0] < @map.elems;
        next unless 0 <= @pos[1] < @map[1].elems;
        next if @map[@pos[0]][@pos[1]] eq "#";
        next if min(@scores[@pos[0]][@pos[1]], 999999999) <= $steps;
        @scores[@pos[0]][@pos[1]] = $steps;
        @to-search.append(
            ((@pos Z+ ( 0,  1)).List, $steps+1),
            ((@pos Z+ ( 0, -1)).List, $steps+1),
            ((@pos Z+ ( 1,  0)).List, $steps+1),
            ((@pos Z+ (-1,  0)).List, $steps+1),
        );
    }
    # say @scores;
    return @scores[*-1][*-1];
}

sub find-closing-byte(@map, @bytes) {
    my $half = @bytes.elems div 2;
    return 0 if @bytes.elems == 1;
    my @new-map = @map>>.Array;
    simulate(@new-map, @bytes[^$half]);
    if find-path-len(@new-map).defined {
        return find-closing-byte(@new-map, @bytes[$half..*]) + $half;
    } else {
        return find-closing-byte(@map, @bytes[^$half]);
    }
}