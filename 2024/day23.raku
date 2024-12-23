sub MAIN($input) {
    my $file = open $input;
    my @connection-list := $file.slurp.trim.lines>>.split("-")>>.List.List;

    my %connections;
    my %all-computers is SetHash;
    for @connection-list -> @c {
        my ($first, $second) = @c.sort;
        %connections{$first} = [] if %connections{$first}:!exists;
        %connections{$second} = [] if %connections{$second}:!exists;
        %connections{$first}.push($second);
        %all-computers{@c.all}++;
    }
    for %connections.values -> $list is rw {
        $list = $list.sort.List;
    }

    my $part1-solution = 0;
    for %connections.keys -> $c1 {
        for %connections{$c1}.Seq -> $c2 {
            for (%connections{$c1} ∩ %connections{$c2}).keys -> $c3 {
                next unless ($c1, $c2, $c3).any.substr(0,1) eq "t";
                $part1-solution++;
            }
        }
    }
    say "part1 solution: $part1-solution";

    my $part2-solution = find-max-party((), %connections, %all-computers).join(",");
    say "part2 solution: $part2-solution";
}

sub find-max-party(@party, %connections, %available-members) {
    my @max-party = @party;
    for %available-members.keys.sort -> $c1 {
        my @new-party := (|@party, $c1);
        my %new-available-members := %available-members ∩ %connections{$c1};
        my @max-party-candidate = find-max-party(@new-party, %connections, %new-available-members);
        @max-party = @max-party-candidate if @max-party-candidate.elems > @max-party.elems;
        last if @max-party.elems == @party.elems + %available-members.elems;
    }
    return @max-party;
}