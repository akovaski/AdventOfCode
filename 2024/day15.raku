sub MAIN($input) {
    grammar Input {
        token TOP { <map> "\n\n" <moves> "\n"* }
        token map { <map-row>+%"\n" }
        token map-row { <[#.O@]>+ }
        token moves { (<[<^>v]>+)+%"\n" }
    }
    my $parsed = Input.parsefile($input);
    my @original-map = $parsed<map><map-row>.map(*.comb.List);
    my @map1 = @original-map>>.Array;
    my @moves = ([~] $parsed<moves>>>.trim).comb;
    my @pos = find-start-pos(@map1);
    
    my %directions = "<" => (0, -1), "^" => (-1, 0), ">" => (0, 1), "v" => (1, 0);
    for @moves -> $move {
        my @d = %directions{$move}.List;
        @pos = move-part1(@map1, "@", @pos, @d);
        # say @map1.map({[~] $_}).join("\n");
    }
    my $part1-solution = 0;
    for ^@map1.elems X ^@map1[0].elems -> ($row, $col) {
        $part1-solution += 100*$row + $col if @map1[$row][$col] eq "O";
    }
    say "part1 solution: $part1-solution";

    my @map2 = @original-map>>.map({
        slip (given $_ {
            when "@" { <@ .> }
            when "O" { <[ ]> }
            default { ($_, $_) }
        })
    })>>.Array;

    @pos = find-start-pos(@map2);

    for @moves -> $move {
        my @d = %directions{$move}.List;
        @pos = move-part2(@map2, "@", @pos, @d);
        # say @map2.map({[~] $_}).join("\n");
    }
    my $part2-solution = 0;
    for ^@map2.elems X ^@map2[0].elems -> ($row, $col) {
        $part2-solution += 100*$row + $col if @map2[$row][$col] eq "[";
    }
    say "part2 solution: $part2-solution";
}

sub find-start-pos(@map) {
    for ^@map.elems X ^@map[0].elems -> ($row, $col) {
        return ($row, $col) if @map[$row][$col] eq "@";
    }
    return Nil;
}

sub move-part1(@map, $piece, @pos, @d) {
    my @moved = @pos Z+ @d;
    given @map[@moved[0]][@moved[1]] {
        when "." {
            @map[@pos[0]][@pos[1]] = ".";
            @map[@moved[0]][@moved[1]] = $piece;
            @moved
        }
        when "#" { @pos }
        when "O" { if (move-part1(@map, "O", @moved, @d) Z== @moved).any == False {
            @map[@pos[0]][@pos[1]] = ".";
            @map[@moved[0]][@moved[1]] = $piece;
            @moved
        } else {
            @pos
        }}
    }
}

sub move-part2(@map, $piece, @pos, @d) {
    my @moved = @pos Z+ @d;
    given @map[@moved[0]][@moved[1]] {
        when "." {
            @map[@pos[0]][@pos[1]] = ".";
            @map[@moved[0]][@moved[1]] = $piece;
            @moved
        }
        when "#" { @pos }
        when ("[" | "]") {
            if @d[0] == 0 {
                if (move-part2(@map, @map[@moved[0]][@moved[1]], @moved, @d) Z== @moved).any == False {
                    @map[@pos[0]][@pos[1]] = ".";
                    @map[@moved[0]][@moved[1]] = $piece;
                    @moved
                } else {
                    @pos
                }
            } else {
                if can-move-vertical-part2(@map, @pos, @d) {
                    move-vertical-unconditional-part2(@map, "@", @pos, @d);
                    @moved
                } else {
                    @pos
                }
            }
        }
    }
}

sub can-move-vertical-part2(@map, @pos, @d) {
    my @moved = @pos Z+ @d;
    given @map[@moved[0]][@moved[1]] {
        when "." { True }
        when "#" { False }
        when "[" {
            my @pair-pos = @moved Z+ (0, 1);
            can-move-vertical-part2(@map, @moved, @d) && can-move-vertical-part2(@map, @pair-pos, @d)
        }
        when "]" {
            my @pair-pos = @moved Z- (0, 1);
            can-move-vertical-part2(@map, @moved, @d) && can-move-vertical-part2(@map, @pair-pos, @d)
        }
    }
}

sub move-vertical-unconditional-part2(@map, $piece, @pos, @d) {
    my @moved = @pos Z+ @d;
    given @map[@moved[0]][@moved[1]] {
        when "[" {
            my @pair-pos = @moved Z+ (0, 1);
            move-vertical-unconditional-part2(@map, "[", @moved, @d);
            move-vertical-unconditional-part2(@map, "]", @pair-pos, @d);
        }
        when "]" {
            my @pair-pos = @moved Z- (0, 1);
            move-vertical-unconditional-part2(@map, "]", @moved, @d);
            move-vertical-unconditional-part2(@map, "[", @pair-pos, @d);
        }
    }
    @map[@pos[0]][@pos[1]] = ".";
    @map[@moved[0]][@moved[1]] = $piece;
}