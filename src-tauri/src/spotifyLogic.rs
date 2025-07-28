// When defining commands in a separate module they should be marked as pub.
// I think I need to use the async commands for the long operations, so they
// run in the background. You can't use "borrow functions" or references to
// data in async functions
