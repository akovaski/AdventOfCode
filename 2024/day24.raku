sub MAIN($input) {
    grammar Input {
        token TOP { <wire>+%"\n" "\n\n" <gate>+%"\n" "\n"* }
        token wire { (\w+) ": " (\d) }
        token gate { (\w+) " " (\w+) " " (\w+) " -> " (\w+) }
    }
    my $parsed = Input.parsefile($input);
    my %initial-wires = $parsed<wire>.map({.[0].Str => .[1].Int.Bool});
    my %gates = $parsed<gate>.map({.[3].Str => (.[1].Str, (.[0].Str, .[2].Str).sort.List)});

    my $part1-solution = wires-to-int(%initial-wires, %gates, "z");
    say "part1 solution: $part1-solution";

    # z00   = XOR(x00, y00)
    # z00c  = AND(y00, x00)
    #
    # z01p  = XOR(x01, y01) # partial addition
    # z01   = XOR(z01p, z00c) # addition including carry-int
    # z01nc = AND(x01, y01) # normal x+y carry over
    # z01cc = AND(z01p, z00c) # x|y + c-in carry over
    # z01c  = OR(z01nc, z01cc) # carry-out. Only one branch can be true, so a XOR would work as well
    #
    # z02p  = XOR(x02, y02)
    # z02   = XOR(z02p, z001c)
    # z02nc = AND(x02, y02)
    # z02cc = AND(z02p, z01c)
    # z02c  = OR(z02nc, z02cc)

    # Only the gate outputs are swapped, in other words the inputs are always correct
    my %unknown-outputs := %gates.keys.SetHash;
    my %known-wrong-outputs is SetHash;
    my %zXp-outputs;
    my %zX-outputs;
    my %zXnc-outputs;
    my %zXcc-outputs;
    my %zXc-outputs;

    for %unknown-outputs.keys -> $uo {
        my ($op, $input-wires) = %gates{$uo};
        if $op eq "XOR" {
            my $is-xy-inputs = $input-wires[0].substr(0,1) eq "x";
            if $is-xy-inputs {
                %zXp-outputs{$uo} = $input-wires[0].substr(1,2).Int;
            } else {
                # any XOR gates that are not for zXp are for zX
                # but we don't know for sure what zX gate it is for
                %zX-outputs{$uo} = Nil;
            }
            %unknown-outputs{$uo}--;
        }
        if $op eq "AND" {
            my $is-xy-inputs = $input-wires[0].substr(0,1) eq "x";
            if $is-xy-inputs {
                %zXnc-outputs{$uo} = $input-wires[0].substr(1,2).Int;
            } else {
                # any AND gates that are not for zXnc are for zXcc
                # but we don't know for sure what zXcc gate it is for
                %zXcc-outputs{$uo} = Nil;
            }
            %unknown-outputs{$uo}--;
        }
        if $op eq "OR" {
            %zXc-outputs{$uo} = Nil;
            %unknown-outputs{$uo}--;
        }
    }
    # z00, z00c are special
    # leaving z00 off of zX-outputs and putting z00nc onto zXc-outputs is useful
    # maybe?
    for %gates.keys -> $wire {
        %zXc-outputs{$wire} = Nil if %zXnc-outputs{$wire}:exists && %zXnc-outputs{$wire} == 0;
    }
    # Let's just look for structural issues, having the wrong inputs for a given gate type
    # A zXnc input would be invalid for a XOR gate
    # A zXp input would be invalid for an OR gate
    # Two gates of the same type would be invalid for all, but it'd be hard to tell which on is wrong
    for %zX-outputs.keys -> $wire {
        if $wire !~~ /^z\d\d$/ {
            # All zX gates should output to a zX wire
            %known-wrong-outputs{$wire}++;
        }
        my $found-zXp = Nil;
        my $found-zXc = Nil;
        for %gates{$wire}[1].Seq {
            if %zXp-outputs{$_}:exists {
                $found-zXp = True;
            } elsif %zXc-outputs{$_}:exists {
                $found-zXc = True;
            } else {
                %known-wrong-outputs{$_}++;
            }
        }
    }
    for %gates.keys -> $wire {
        if %zX-outputs{$wire}:exists && $wire.substr(0,1) ne "z" {
            %known-wrong-outputs{$wire}++;
        }
    }
    for %zXc-outputs.keys -> $wire {
        next if %zXnc-outputs{$wire}:exists && %zXnc-outputs{$wire} == 0;
        my $found-zXnc = Nil;
        my $found-zXcc = Nil;
        for %gates{$wire}[1].Seq {
            if %zXnc-outputs{$_}:exists {
                $found-zXnc = True;
            } elsif %zXcc-outputs{$_}:exists {
                $found-zXcc = True;
            } else {
                %known-wrong-outputs{$_}++;
            }
        }
    }
    for %zXcc-outputs.keys -> $wire {
        my $found-zXp = Nil;
        my $found-zXc = Nil;
        for %gates{$wire}[1].Seq {
            if %zXp-outputs{$_}:exists {
                $found-zXp = True;
            } elsif %zXc-outputs{$_}:exists {
                $found-zXc = True;
            } else {
                %known-wrong-outputs{$_}++;
            }
        }
    }
    # zXp wires should appear 2 times each
    # zXnc wires should appear 1 time each
    # zXc wires outputs should appear 2 times each
    # zXcc wires should appear 1 time each
    # zX wires should appear 0 time each
    my %wire-to-created-outputs;
    for %gates.keys -> $output {
        my ($op, $input-wires) = %gates{$output};
        for $input-wires.Seq {
            %wire-to-created-outputs.push($_ => $output);
        }
    }
    for %gates.keys -> $wire {
        my $created-outputs = %wire-to-created-outputs{$wire}:exists ?? %wire-to-created-outputs{$wire}.elems !! 0;
        my $was = %known-wrong-outputs{$wire};
        %known-wrong-outputs{$wire}++ if %zXp-outputs{$wire}:exists && $created-outputs != 2 && $wire ne "z00";
        %known-wrong-outputs{$wire}++ if %zXnc-outputs{$wire}:exists && $created-outputs != 1 && %zXnc-outputs{$wire} != 0;
        %known-wrong-outputs{$wire}++ if %zXc-outputs{$wire}:exists && $created-outputs != 2;
        %known-wrong-outputs{$wire}++ if %zXcc-outputs{$wire}:exists && $created-outputs != 1;
        %known-wrong-outputs{$wire}++ if %zX-outputs{$wire}:exists && $created-outputs != 0;
    }
    %known-wrong-outputs{"z45"}--; # z45 is fine, but it uses OR instead of XOR, because there is no x45/y45
    my $part2-solution = %known-wrong-outputs.keys.sort.join(",");

    say "part2-solution: $part2-solution";
}

sub wires-to-int(%wires, %gates, $prefix) {
    my $int = 0;
    if $prefix eq <x y>.any {
        for %wires.keys {
            next if .substr(0,1) ne $prefix;
            $int += 1 +< .substr(1,*).Int if %wires{$_};
        }
    } else {
        for %gates.keys {
            next if .substr(0,1) ne $prefix;
            resolve-gates(%wires, %gates, $_);
            $int += 1 +< .substr(1,*).Int if %wires{$_};
        }
    }
    return $int;
}

sub resolve-gates(%wires, %gates, $gate) {
    my ($op, @input-wires) := %gates{$gate};
    my @input-values = @input-wires.map({resolve-gates(%wires, %gates, $_) if %wires{$_}:!exists; %wires{$_}});
    %wires{$gate} = (given $op {
        when "AND" { [&&] @input-values }
        when "XOR" { [!=] @input-values }
        when "OR"  { [||] @input-values }
    });
}
