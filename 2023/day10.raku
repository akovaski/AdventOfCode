use v6;

sub MAIN($input) {
    my $file = open $input;

    my @map;
    for $file.lines -> $line {
        @map.push($line.comb.List);
    }

    my @starting-point;
    FIND_S: for ^@map.elems -> $y {
        my @row := @map[$y];
        for ^@row.elems -> $x {
            if @row[$x] eq 'S' {
                @starting-point = ($y, $x);
                last FIND_S;
            }
        }
    }

    my @path = (@starting-point,);
    my $len = 0;

    my @colored-map;
    repeat {
        my $tile = @map[@path.tail[0]][@path.tail[1]];
        @colored-map[@path.tail[0]][@path.tail[1]] = 9;
        if @path.elems >= 2 {
            my @direction = @path.tail.List Z- @path[*-2].List;
            #rotate 90 degrees counter clockwise
            my @rotated = (-@direction[1], @direction[0]);
            my @color1;
            @color1.push(@path.tail.List Z+ @rotated);
            my @color2;
            @color2.push(@path.tail.List Z- @rotated);
            given @direction {
                when (1,0) {
                    @color1.push(@path.tail.List Z+ @direction) if $tile eq 'J';
                    @color2.push(@path.tail.List Z+ @direction) if $tile eq 'L';
                }
                when (-1,0) {
                    @color1.push(@path.tail.List Z+ @direction) if $tile eq 'F';
                    @color2.push(@path.tail.List Z+ @direction) if $tile eq '7';
                }
                when (0,1) {
                    @color1.push(@path.tail.List Z+ @direction) if $tile eq '7';
                    @color2.push(@path.tail.List Z+ @direction) if $tile eq 'J';
                }
                when (0,-1) {
                    @color1.push(@path.tail.List Z+ @direction) if $tile eq 'L';
                    @color2.push(@path.tail.List Z+ @direction) if $tile eq 'F';
                }
                default {
                    note "don't like this direction @direction.raku()";
                    exit();
                }
            }
            for @color1 -> @color1 {
                if all(@color1 Z>= (0, 0)) && all(@color1 Z<= (@map.end, @map.head.end)) && (@colored-map[@color1[0]][@color1[1]]:!exists) {
                    @colored-map[@color1[0]][@color1[1]] = 1;
                }
            }
            for @color2 -> @color2 {
            if all(@color2 Z>= (0, 0)) && all(@color2 Z<= (@map.end, @map.head.end)) && (@colored-map[@color2[0]][@color2[1]]:!exists) {
                @colored-map[@color2[0]][@color2[1]] = 2;
            }
            }
        }
        my @neighbors = do given $tile {
            when '|' {((1,0),(-1,0))}
            when '-' {((0,-1),(0,1))}
            when 'L' {((-1,0),(0,1))}
            when 'J' {((-1,0),(0,-1))}
            when '7' {((0,-1),(1,0))}
            when 'F' {((1,0),(0,1))}
            when 'S' {((1, 0), (-1, 0), (0, 1), (0, -1))}
            default {
                note "I don't like this tile";
                exit();
            }
        };
        for @neighbors -> @neighbor {
            my @neighbor-position = @neighbor Z+ @path.tail.List;
            next if @neighbor-position[0] < 0 || @neighbor-position[0] > @map.end;
            next if @neighbor-position[1] < 0 || @neighbor-position[1] > @map.head.end;
            next if @path.elems >= 2 && @neighbor-position eqv @path[*-2];

            my $neighbor-tile = @map[@neighbor-position[0]][@neighbor-position[1]];
            if @neighbor eqv ( 1,0) && $neighbor-tile eq <S | L J>.any
            || @neighbor eqv (-1,0) && $neighbor-tile eq <S | 7 F>.any
            || @neighbor eqv (0, 1) && $neighbor-tile eq <S - J 7>.any
            || @neighbor eqv (0,-1) && $neighbor-tile eq <S - L F>.any {
                @path.push(@neighbor-position);
                last;
            }
        }
        if $len == @path.elems {
            exit();
        }
        $len = @path.elems;
    } while @path.tail !eqv @path.head;
    my $part-one-solution = (@path.elems - 1) / 2;
    say "part 1: {$part-one-solution}";

    my $newly-colored;
    repeat {
        $newly-colored = 0;
        for ^@map.elems -> $y {
            for ^@map.head.elems -> $x {
                if @colored-map[$y][$x]:!exists {
                    my @neighbor-colors = ((1, 0), (-1, 0), (0, 1), (0, -1))
                        .map({$_ Z+ ($y, $x)})
                        .map({@colored-map[$_[0]][$_[1]]})
                        .grep({$_.defined && $_ != 9});
                    with @neighbor-colors.head() {
                        @colored-map[$y][$x] = $_;
                        $newly-colored += 1;
                    }
                }
            }
        }
    } until $newly-colored == 0;

    # say @colored-map.map({$_.map({$_ || ' '}).join}).join("\n");
    my $inside-color = @colored-map[0].tail == 2 ?? 1 !! 2;
    my $part-two-solution = @colored-map.map({$_.grep($inside-color).elems}).sum;
    say "part 2: $part-two-solution";
}