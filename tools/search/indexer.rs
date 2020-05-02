#[macro_use]
extern crate flags;

use indexer_lib::{
    AggregateDefinitionsFn, AggregateTrigramsFn, ExtractCandidatesFn, ExtractDefinitionsFn,
    ExtractTrigramsFn, ProcessFilesFn,
};
use plume::{EmitFn, PCollection, Stream, StreamingIterator, KV};

fn fail(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn main() {
    let input_dir = define_flag!("input_dir", String::new(), "The directory to read from");
    let input_recordio = define_flag!(
        "input_recordio",
        String::new(),
        "The code recordio to read from. If provided, won't generate one"
    );
    let output_dir = define_flag!(
        "output_dir",
        String::new(),
        "The directory to write the index to"
    );

    parse_flags!(input_dir, output_dir, input_recordio);

    if output_dir.path().is_empty() {
        fail("You must specify an --output to write to!");
    }

    let starting_dir = if input_dir.path().is_empty() {
        std::env::current_dir().unwrap()
    } else {
        input_dir.path().into()
    };

    // Extract the codebase into a code sstable
    let code_recordio = if input_recordio.path().is_empty() {
        let code_recordio = format!("{}/code.recordio", output_dir.path());
        extract_lib::extract_code(&starting_dir, &code_recordio);
        code_recordio
    } else {
        input_recordio.path()
    };

    // Interpret filetypes and process file data
    let code = PCollection::from_recordio(&code_recordio);
    let files = code.par_do(ProcessFilesFn {});
    let files_sstable = format!("{}/files.sstable", output_dir.path());
    files.write_to_sstable(&files_sstable);

    // Extract file info by file_id
    let code = PCollection::from_recordio(&code_recordio);
    let files = code.par_do(ExtractCandidatesFn {});
    let files_sstable = format!("{}/candidates.sstable", output_dir.path());
    files.write_to_sstable(&files_sstable);

    // Extract trigrams
    let code = PCollection::from_recordio(&code_recordio);
    let trigrams = code.par_do(ExtractTrigramsFn {});
    let trigram_matches = trigrams.group_by_key_and_par_do(AggregateTrigramsFn {});
    let trigrams_sstable = format!("{}/trigrams.sstable", output_dir.path());
    trigram_matches.write_to_sstable(&trigrams_sstable);

    // Extract definitions
    let code = PCollection::from_recordio(&code_recordio);
    let keywords = code.par_do(ExtractDefinitionsFn {});
    let mut index = keywords.group_by_key_and_par_do(AggregateDefinitionsFn {});

    let definitions_sstable = format!("{}/definitions.sstable", output_dir.path());
    index.write_to_sstable(&definitions_sstable);

    plume::run();
}
