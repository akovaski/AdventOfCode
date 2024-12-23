sub MAIN($input) {
    my $file = open $input;
    my @connection-list := $file.slurp.trim.lines>>.split("-")>>.List.List;

    my %connections;
    for @connection-list -> @c {
        %connections{@c[0]} = [] if %connections{@c[0]}:!exists;
        %connections{@c[1]} = [] if %connections{@c[1]}:!exists;
        %connections{@c[0]}.push(@c[1]);
        %connections{@c[1]}.push(@c[0]);
    }
    for %connections.values -> $list is rw {
        $list = $list.sort.List;
    }

    my $part1-solution = 0;
    for %connections.keys -> $c1 {
        for %connections{$c1}.Seq -> $c2 {
            next if $c2 lt $c1;
            for %connections{$c2}.Seq  -> $c3 {
                next if $c3 lt $c2;
                next unless $c3 (elem) %connections{$c1}.List;
                next unless ($c1, $c2, $c3).any.substr(0,1) eq "t";
                $part1-solution++;
            }
        }
    }
    say "part1 solution: $part1-solution";

    my $part2-solution = find-max-party((), %connections, %connections.keys.Set).join(",");
    say "part2 solution: $part2-solution";
}

sub find-max-party(@party, %connections, %available-members) {
    my @max-party = @party;
    for %available-members.keys -> $c1 {
        my @new-party := (|@party, $c1);
        my %new-available-members := %available-members ∩ %connections{$c1}.grep({$_ gt $c1});
        my @max-party-candidate = find-max-party(@new-party, %connections, %new-available-members);
        @max-party = @max-party-candidate if @max-party-candidate.elems > @max-party.elems;
    }
    return @max-party;
}