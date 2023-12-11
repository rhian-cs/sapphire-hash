use recursive_hash_calculator_core::hasher::HasherResult;

pub fn format(result: &HasherResult, output_path: &String) -> String {
    let count = result.processed_files_count;
    let elapsed_time = result.elapsed_time.as_secs_f32();
    let files_verb = files_verb_for(count);

    [
        format!("{count} {files_verb} been processed!"),
        format!("Took {elapsed_time} seconds."),
        format!("The report has been saved at:\n{output_path}"),
    ]
    .join("\n\n")
}

fn files_verb_for(count: i32) -> &'static str {
    if count == 1 {
        "file has"
    } else {
        "files have"
    }
}
