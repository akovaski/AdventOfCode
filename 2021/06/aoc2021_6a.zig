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

    var schedule: [10]i64 = .{0} ** 10;
    while (try input.readUntilDelimiterOrEof(&buf, ',')) |num_s| {
        const number = try std.fmt.parseInt(usize, num_s, 10);
        schedule[number] += 1;
    }
    //std.log.info("schedule: {d}", .{schedule});

    // run the laternfish simulation!
    var day: usize = 0;
    while (day < 80) {
        const sched_day = day % schedule.len;
        const todays_fishies = schedule[sched_day];
        const old_fishies_day = (day + 7) % schedule.len;
        const new_fishies_day = (day + 9) % schedule.len;
        schedule[sched_day] = 0;
        schedule[old_fishies_day] += todays_fishies;
        schedule[new_fishies_day] += todays_fishies;
        day += 1;
    }
    //std.log.info("schedule: {d}", .{schedule});

    var sum: i64 = 0;
    for (schedule) |val| {
        sum += val;
    }
    std.log.info("number of lanternfish: {}", .{sum});
}
