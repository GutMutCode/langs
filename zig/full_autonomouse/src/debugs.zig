const std = @import("std");
const structs = @import("./structs.zig");

pub fn mod_test() void {
    std.debug.print("Debug, {s}!\n", .{"this"});
}
