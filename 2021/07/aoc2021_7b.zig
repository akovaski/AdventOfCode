const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    const input_file_name = if (args.len >= 2) args[1] else "input.txt";

    const input_file = try std.fs.cwd().openFile(input_file_name, .{ .read = true });
    const input = input_file.reader();
    var buf: [300]u8 = undefined;

    var num_crabs: usize = 0;
    while ((try input.readUntilDelimiterOrEof(&buf, ',')) != null) {
        num_crabs += 1;
    }

    var crabs: []i16 = try allocator.alloc(i16, num_crabs);
    try input_file.seekTo(0);
    for (crabs) |*crab| {
        const crab_s = (try input.readUntilDelimiterOrEof(&buf, ',')).?;
        crab.* = try std.fmt.parseInt(i16, crab_s, 10);
    }

    //std.log.info("crabs: {d}", .{crabs});

    const median = quickSelect(crabs, crabs.len / 2);
    const efficient_pos = try searchForEfficientPosition(crabs, median);
    const fuel_cost = try getFuelCost(crabs, efficient_pos);
    std.log.info("most effecient position: {}, cost: {}", .{ efficient_pos, fuel_cost });
}

fn searchForEfficientPosition(crabs: []i16, guess_pos: i16) !i16 {
    const guess_fuel_cost: i64 = try getFuelCost(crabs, guess_pos);
    const up_fuel_cost: i64 = try getFuelCost(crabs, guess_pos + 1);
    const down_fuel_cost: i64 = try getFuelCost(crabs, guess_pos - 1);
    //std.log.info("fuel cost {}: {} - {} - {}", .{ guess_pos, down_fuel_cost, guess_fuel_cost, up_fuel_cost });

    // search for the most efficient position
    if (down_fuel_cost < guess_fuel_cost) {
        return searchForEfficientPosition(crabs, guess_pos - 1);
    } else if (up_fuel_cost < guess_fuel_cost) {
        return searchForEfficientPosition(crabs, guess_pos + 1);
    } else {
        return guess_pos;
    }
}

fn getFuelCost(crabs: []i16, pos: i16) !i64 {
    var fuel_cost: i64 = 0;
    for (crabs) |crab| {
        const distance: i64 = try std.math.absInt(pos - crab);
        // triangular number sequence: size of triangle = n*(n+1)/2
        fuel_cost += @divExact(distance * (distance + 1), 2);
    }
    return fuel_cost;
}

fn pickPivot(list: []i16) usize {
    return list.len / 2;
}

// select the k'th smallest element
// for example, if k==0 selects the smallest element from the list
fn quickSelect(list: []i16, k: usize) i16 {
    if (list.len == 1) {
        std.debug.assert(k == 0);
        return 0;
    }
    const pivot = list[pickPivot(list)];

    var lows: usize = 0;
    var highs: usize = 0;
    var pivots: usize = 0;
    var read_i: usize = 0;
    while (read_i + highs < list.len) {
        const val = list[read_i];
        if (val < pivot) {
            list[lows] = val;
            lows += 1;
            read_i += 1;
        } else if (val > pivot) {
            list[read_i] = list[list.len - 1 - highs];
            list[list.len - 1 - highs] = val;
            highs += 1;
        } else {
            pivots += 1;
            read_i += 1;
        }
    }

    {
        // restore pivots
        var i = lows;
        while (i < lows + pivots) {
            list[i] = pivot;
            i += 1;
        }
    }
    //std.log.info("qs l: {any} k: {}, pivot: {}, lows: {}, pivots: {}, highs: {}", .{ list, k, pivot, lows, pivots, highs });

    if (k < lows) {
        return quickSelect(list[0..lows], k);
    } else if (k < lows + pivots) {
        return pivot;
    } else {
        return quickSelect(list[lows + pivots ..], k - lows - pivots);
    }
}
