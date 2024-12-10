sub MAIN($input) {
    my $file = open $input;
    my @map = $file.slurp.trim.comb>>.Int;

    my @rotored = @map.rotor(2, :partial);
    my @file-sizes = (@rotored[*;0] Z (0..*))>>.Array;
    my @free-sizes = @rotored[*;1].grep(so *);

    my $is-file = True;

    # # I'm not sure why this doesn't work
    # my $part1-solution = 0;
    # my $pos = 0;
    # my $compacted-size = @file-sizes[*;0].sum;
    # while $pos < $compacted-size {
    #     my $id;
    #     my $size;
    #     # say "pos $pos, total $compacted-size";
    #     # say "file-sizes {@file-sizes.raku}";
    #     # say "free-sizes {@free-sizes.raku}";
    #     # say "\tisfile $is-file";
    #     if $is-file {
    #         my $disk-file = @file-sizes.shift;
    #         $id = $disk-file[1];
    #         $size = $disk-file[0];
    #         $is-file = False;
    #     } else {
    #         $id = @file-sizes[*-1][1];
    #         $size;
    #         if @file-sizes[*-1][0] == @free-sizes[0] {
    #             $size = @free-sizes.shift;
    #             @file-sizes.pop;
    #             $is-file = True;
    #         } elsif @file-sizes[*-1][0] > @free-sizes[0] {
    #             # say "complete free";
    #             $size = @free-sizes.shift;
    #             # say "fs {@file-sizes[*-1]}";
    #             @file-sizes[*-1][0] -= $size;
    #             $is-file = True;
    #         } else {
    #             # say "partial free";
    #             $size = @file-sizes.pop[0];
    #             @free-sizes[0] -= $size;
    #             $is-file = False;
    #         }
    #     }
    #     # Probably an entirely unncessary optimization
    #     # (pos+0)*id + (pos+1)*id + (pos+2)*id ... + (pos+size-1)*id
    #     # id*(pos+0 + pos+1 + pos+2 ... + pos+size-1)
    #     # id*(size*pos + 0+1+2 ... + size-1)
    #     # id*(size*pos + (size-1)*size/2)
    #     # say "\tid $id, size $size, pos $pos";
    #     $part1-solution += $id * ($size*$pos + (($size-1)*$size div 2));
    #     $pos += $size;
    #     # say "\tsum $part1-solution";
    # }

    my $total-size = @map.sum;
    my @drive-map;
    my $block-id = 0;
    for @map -> $size {
        my $mark;
        if $is-file {
            $mark = $block-id;
            $block-id += 1;
        } else {
            $mark = -1;
        }
        for 0..^$size {
            @drive-map.push($mark);
        }
        $is-file = not $is-file;
    }
    my $r-pos = @drive-map.elems-1;
    for 0..* -> $pos {
        last if $pos >= $r-pos;
        if @drive-map[$pos] == -1 {
            for 0..* {
                last if $pos >= $r-pos;
                if @drive-map[$r-pos] != -1 {
                    @drive-map[$pos] = @drive-map[$r-pos];
                    @drive-map[$r-pos] = -1;
                    $r-pos -= 1;
                    last;
                }
                $r-pos -= 1;
            }
        }
    }
    my $part1-solution = (@drive-map.grep(* != -1) Z* (0..*)).sum;
    say "part 1: $part1-solution";

    #slow solution for part 2
    @drive-map = [];
    $block-id = 0;
    my %initial-file-pos;
    $is-file = True;
    for @map -> $size {
        my $mark;
        if $is-file {
            $mark = $block-id;
            %initial-file-pos{$block-id} = (@drive-map.elems, $size);
            $block-id += 1;
        } else {
            $mark = -1;
        }
        for 0..^$size {
            @drive-map.push($mark);
        }
        $is-file = not $is-file;
    }
    for $block-id^...0 -> $block-to-move {
        # say "checking $block-to-move";
        my $last-pos-to-check = %initial-file-pos{$block-to-move}[0];
        my $block-size = %initial-file-pos{$block-to-move}[1];
        my $free-count = 0;
        for 0..$last-pos-to-check -> $check-pos {
            if @drive-map[$check-pos] == -1 {
                $free-count++;
            } else {
                $free-count = 0;
            }
            if $free-count == $block-size {
                @drive-map[(0..^$block-size).map($check-pos - *)] = $block-to-move xx $block-size;
                @drive-map[(0..^$block-size).map($last-pos-to-check + *)] = -1 xx $block-size;
                last;
            }
        }
    }
    # say @drive-map.map({if $_ == -1 { "." } else { $_ }}).join();
    my $part2-solution = (@drive-map.map({max(0,$_)}) Z* (0..*)).sum;
    say "part 2: $part2-solution";
}