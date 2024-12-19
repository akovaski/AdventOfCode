sub MAIN($input) {
    grammar Input {
        token TOP { <robot>+%"\n" "\n"* }
        token robot { <position> " " <velocity> }
        token position { "p=" (<[\d-]>+) "," (<[\d-]>+) }
        token velocity { "v=" (<[\d-]>+) "," (<[\d-]>+) }
    }
    my $parsed = Input.parsefile($input);
    my $robots = $parsed<robot>.map({Map.new('p' => .<position>>>.Int.List, 'v' => .<velocity>>>.Int.List)}).List;

    my @room = (101, 103);

    my $new-positions = wait($robots, @room, 100);
    my $quadrants = [Z+] $new-positions.map({
        my ($w, $h) = @room.map(* div 2);
        (.[0] < $w && .[1] < $h, .[0] < $w && .[1] > $h, .[0] > $w && .[1] < $h, .[0] > $w && .[1] > $h)
    }).List;

    my $part1-solution = [*] $quadrants;
    say "part1 solution: $part1-solution";

    for 0..* {
        my $max-bag = check-max-bag(wait($robots, @room, $_));
        if $max-bag == 1 {
            display(wait($robots, @room, $_), @room);
            say "part2 solution: $_";
            last;
        }
    }
}

sub wait($robots, @room, $seconds) {
    $robots.map({.<p> Z+ .<v>.map(* * $seconds)})>>.List.map({ $_ Z- (($_ Zdiv @room) Z* @room) })>>.List.List
}

sub check-max-bag($positions) {
    $positions.map({.[0] ~ "," ~ .[1]}).Bag.values.max
}

sub display($positions, @room) {
    # say $positions.map({.[0] ~ "," ~ .[1]}).raku;
    my %positions = $positions.map({.[0] ~ "," ~ .[1]}).Set;
    # say %positions.raku;
    my $image = "";
    for [X] @room.reverse.map(^*+1) -> ($y, $x) {
        if %positions{"$x,$y"} {
            $image ~= "X";
        } else {
            $image ~= ".";
        };
        $image ~= "\n" if $x == @room[0];
    }
    print $image;
}