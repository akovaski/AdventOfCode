sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.lines>>.comb>>.List.List;
    my %freqs;
    for 0..^@map.elems -> $row {
        for 0..^@map[0].elems -> $col {
            if @map[$row; $col] ne "." {
                my $freq = @map[$row; $col];
                %freqs{$freq} = [] if %freqs{$freq}:!exists;
                %freqs{$freq}.push(($row, $col));
            }
        }
    }
    my %antinodes is SetHash;
    for %freqs.kv -> $freq, @locations {
        for (0..^@locations.elems) X (0..^@locations.elems) -> ($loc1, $loc2) {
            next if $loc1 == $loc2;
            my @base = @locations[$loc1].List;
            my @vector = @locations[$loc2].List Z- @base;
            my @antinode1 = @base Z+ @vector.map(* * 2);
            %antinodes{@antinode1.List.raku}++ if point-is-in-map(@map, @antinode1);
            my @antinode2 = @base Z+ @vector.map(* * -1);
            %antinodes{@antinode2.List.raku}++ if point-is-in-map(@map, @antinode2);
        }
    }
    my $part1-solution = %antinodes.elems;
    say "part 1: $part1-solution";


    my %antinodes2 is SetHash;
    for %freqs.kv -> $freq, @locations {
        for (0..^@locations.elems) X (0..^@locations.elems) -> ($loc1, $loc2) {
            next if $loc1 == $loc2;
            my @base = @locations[$loc1].List;
            my @vector = @locations[$loc2].List Z- @base;
            # make integer unit-ish vector
            for 2..@vector[0] -> $divisor {
                if @vector[0] %% $divisor and @vector[1] %% $divisor {
                    @vector[0] = @vector[0] div $divisor;
                    @vector[1] = @vector[1] div $divisor;
                }
            }
            for 0..max(@map.elems, @map[0].elems) -> $length {
                my @antinode = @base Z+ @vector.map(* * $length);
                if point-is-in-map(@map, @antinode) {
                    %antinodes2{@antinode.List.raku}++ 
                } else {
                    last
                }
            }
            for 1..max(@map.elems, @map[0].elems) -> $length {
                my @antinode = @base Z+ @vector.map(* * -$length);
                if point-is-in-map(@map, @antinode) {
                    %antinodes2{@antinode.List.raku}++ 
                } else {
                    last
                }
            }
        }
    }
    my $part2-solution = %antinodes2.elems;
    say "part 2: $part2-solution";
}

sub point-is-in-map(@map, @point) {
    return False if !(0 <= @point[0] < @map.elems);
    return False if !(0 <= @point[1] < @map[0].elems);
    return True;
}