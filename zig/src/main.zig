const std = @import("std");

pub fn main() !void {
    const file_name = "../data/day02.txt";

    // At runtime:
    {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        const file_content = try std.fs.cwd().readFileAlloc(gpa.allocator(), file_name, std.math.maxInt(usize));
        file_content.
        std.debug.print("{s}\n", .{file_content});
    }

    // At comptime:
    // {
    //     const file_content = @embedFile(file_name);  // broken doesn't seem to like this way can't find the file
    //     std.debug.print("{s}\n", .{file_content});
    // }


    //std.log.info("Testing");
    std.debug.print("test\n",.{});
}
