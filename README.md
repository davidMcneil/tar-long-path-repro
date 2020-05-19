# Tar Long Path Repro

On Windows `tar-rs` fails to unpack an archive when the archive contains a folder containing a file where the name of the folder is long enough to require a `@LongLink`. `test` contains the example directory structure and the example tar archive.

The error can be replicated by running `cargo run`. The underlying error is "Access is denied." By adding logging to `tar` you can see the order of entries processed is as follows:

* "test/expanded/this_is_a_file_with_a_name_that_just_keeps_going_on_and_on_and_on_it_is_a_very_very_long_file_name" `Directory`
* "test/expanded/@LongLink" `GNULongName`
* "test/expanded/this_is_a_file_with_a_name_that_just_keeps_going_on_and_on_and_on_it_is_a_very_very_long_file_name" `Regular`

The third entry is pointing to the directory when it should be pointing to the file. The "Access is denied." error is the result of trying to open a directory as a file.
