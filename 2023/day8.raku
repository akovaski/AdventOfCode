use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar Pouch {
        token TOP { <directions> "\n"+ <nodes> "\n"* }
        token directions { <[LR]>+ }
        token nodes { <node>+%"\n" }
        token node { $<name>=<.id> " = (" $<L>=<.id> ", " $<R>=<.id> ")" }
        token id { \w+ }
    }

    my $pouch = Pouch.parse($file.slurp);
    class Node {
        has $.L;
        has $.R;
    }
    my %graph;
    for $pouch<nodes><node> -> $node {
        %graph{$node<name>.Str} = Node.new(L => $node<L>.Str, R => $node<R>.Str);
    }
    my $position = 'AAA';
    my $part-one-solution = 0;
    PART1: loop {
        for $pouch<directions>.comb -> $LR {
            $part-one-solution += 1;
            my $node = %graph{$position};
            $position = $LR eq 'L' ?? $node.L !! $node.R;
            if $position eq 'ZZZ' {
                last PART1;
            }
        }
    }
    say "part 1: $part-one-solution";

    my @starting-positions = %graph.keys.grep({/.*A/});
    class CycleStats {
        has $.offset is rw;
        has $.length is rw;
        has @.z-indicies is rw;
    }
    my @stats;
    my @directions = $pouch<directions>.comb;
    my $dir-length = $pouch<directions>.chars;
    for @starting-positions -> $start {
        my $position = $start;
        my @position-history = [$start];
        my $cycle-length = 0;
        FIND-CYCLE: loop {
            for @directions -> $LR {
                $cycle-length += 1;
                my $node = %graph{$position};
                $position = $LR eq 'L' ?? $node.L !! $node.R;
                @position-history.push($position);
                for 1..* -> $dir-multiplier {
                    my $prev-index = $cycle-length - $dir-length*$dir-multiplier;
                    last if $prev-index < 0;
                    if $position eq @position-history[$prev-index] {
                        my $cycle = CycleStats.new();
                        $cycle.offset = $prev-index;
                        $cycle.length = $dir-length*$dir-multiplier;

                        for $cycle.offset ...^ @position-history.elems -> $i {
                            my $historic-position = @position-history[$i];
                            if $historic-position ~~ /.*Z/ {
                                $cycle.z-indicies.push($i - $cycle.offset);
                            }
                        }
                        @stats.push($cycle);
                        last FIND-CYCLE;
                    }
                }
            }
        }
    }

    ## Trying to simulate each step: way too slow around 210000 hours of computation
    # my $part-two-solution;
    # my @positions = @starting-positions;
    # my $count;
    # PART2: loop {
    #     for $pouch<directions>.comb -> $LR {
    #         $part-two-solution += 1;
    #         my @nodes = %graph{@positions};
    #         @positions = $LR eq 'L' ?? @nodes».L !! @nodes».R;
    #         say "steps: $part-two-solution" if ($count = ++$count % 65536) == 0;
    #         if (@positions.all ~~ /.*Z/).defined {
    #             last PART2;
    #         }
    #     }
    # }
    # say "part 2: $part-two-solution";

    ## Try analyze the loop cycles and jump each ghost to the next Z: still way too slow, around 10 hours of computation
    # sub z-seq($cycle) {
    #     my $cycle-count = 0;
    #     my $zi = 0;
    #     return {
    #         # say "cycle-count $cycle-count, zi $zi";
    #         my $return = $cycle.offset + $cycle-count*$cycle.length + $cycle.z-indicies[$zi];
    #         if $zi == $cycle.z-indicies.end {
    #             $zi = 0;
    #             $cycle-count += 1;
    #         } else {
    #             $zi += 1;
    #         }
    #         $return
    #     };
    # }
    # my @sequencers = @stats.map({z-seq($_)});
    # my @positions = @sequencers.map({$_()});
    # my @seq-pos = @sequencers Z @positions;
    # sub do-z-increment($max-position, @sp) {
    #     while @sp[1] < $max-position {
    #         @sp[1] = @sp[0]();
    #     }
    # }
    # my $count;
    # loop {
    #     my $max-position = @seq-pos».[1].max;
    #     say "seq-pos: {@seq-pos».[1].raku}" if ($count = ++$count % 65536) == 0;
    #     @seq-pos.map({do-z-increment($max-position, $_)});
    #     last if @seq-pos».[1].all == @seq-pos.head[1];
    # }
    # say "part 2: {@seq-pos.head[1]}";

    # present a solution only for collections that all have exactly 1 z-index in their loop
    # AND the z-position is equal to the loop length
    if @stats.map({$_.z-indicies.elems}).any != 1 && @stats.map({$_.z-indicies.head + $_.offset == $_.length}).all == True {
        say "Invalid input";
        exit();
    }
    my $part-two-solution = [lcm] @stats.map({$_.offset + $_.z-indicies.head});
    say "part 2: $part-two-solution";
}