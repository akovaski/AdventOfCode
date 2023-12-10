use v6;

sub MAIN($input) {
    my $file = open $input;

    my @map = $file.lines».comb».Array;

    my @starting-point = @map».grep('S', :k)».[0].grep(*.defined, :kv).List;

    my @path = (@starting-point,);

    my %tile-neighbors =
        '|' => (( 1, 0),(-1, 0)),
        '-' => (( 0,-1),( 0, 1)),
        'L' => ((-1, 0),( 0, 1)),
        'J' => ((-1, 0),( 0,-1)),
        '7' => (( 1, 0),( 0,-1)),
        'F' => (( 1, 0),( 0, 1)),
    ;

    sub connecting-neighbor(@position, @neighbor) {
        my @neighbor-position = @position Z+ @neighbor;
        return False if any(@neighbor-position Z< (0, 0));
        return False if any(@neighbor-position Z> (@map.end, @map.head.end));
        my $neighbor-tile = @map[@neighbor-position[0]; @neighbor-position[1]];
        my @negative-neighbor = @neighbor X* -1;
        return %tile-neighbors{$neighbor-tile}.grep(@negative-neighbor, :k).elems > 0;
    }

    # replace starting-point with the appropriate pipe
    my @start-tile-candidates = <| - L J 7 F>;
    for @start-tile-candidates -> $candidate {
        next if %tile-neighbors{$candidate}.map({!connecting-neighbor(@starting-point, $_)}).any;
        @map[@starting-point[0]; @starting-point[1]] = $candidate;
        last;
    }

    repeat {
        my @position := @path.tail;
        my $tile = @map[@position[0]; @position[1]];
        my @neighbors = %tile-neighbors{$tile}.List;
        for @neighbors -> @neighbor {
            my @neighbor-position = @neighbor Z+ @position;
            next if @path.elems >= 2 && @neighbor-position eqv @path[*-2];
            if connecting-neighbor(@position, @neighbor) {
                @path.push(@neighbor-position);
                last;
            }
        }
    } while @path.tail !eqv @path.head;
    my $part-one-solution = (@path.elems / 2).floor;
    say "part 1: {$part-one-solution}";

    my %pipe-set = @path.Set;
    my %same-side-pairs = <F 7 L J>;
    my $part-two-solution = 0;
    for ^@map.elems -> $y {
        my $inside = False;
        my $entrance-pipe = Nil;
        for ^@map.head.elems -> $x {
            if %pipe-set{$($y, $x)} {
                given @map[$y; $x] {
                    when '|' { $inside = !$inside }
                    when 'F' | 'L' { $entrance-pipe = $_ }
                    when 'J' | '7' {
                        $inside = !$inside if %same-side-pairs{$entrance-pipe} ne $_;
                        $entrance-pipe = Nil;
                    }
                }
            } elsif $inside {
                $part-two-solution += 1;
            }
        }
    }
    say "part 2: $part-two-solution";
}