extern crate cache;
extern crate largetable_client;
extern crate weld;

use std::collections::HashMap;
use std::io::prelude::*;
use std::process::Command;
use std::sync::Arc;
use std::sync::RwLock;

use cache::Cache;
use weld::File;

const CHANGES: &'static str = "changes";
const CHANGE_METADATA: &'static str = "metadata";
const CHANGE_IDS: &'static str = "change_ids";
const SNAPSHOTS: &'static str = "snapshots";
const SNAPSHOT_IDS: &'static str = "snapshots_ids";
const CACHE_SIZE: usize = 4096;

#[derive(Clone)]
pub struct Repo<C: largetable_client::LargeTableClient, W: weld::WeldServer> {
    db: C,
    pub remote_server: Option<W>,
    cache: Arc<Cache<ReadQuery, ReadResponse>>,

    // Map of friendly name to support change lookup by friendly name.
    spaces: Arc<RwLock<HashMap<String, u64>>>,
}

#[derive(Clone, PartialEq, Hash)]
enum ReadQuery {
    Read(u64, String, u64),
    ListFiles(u64, String, u64),
}

impl Eq for ReadQuery {}

#[derive(Clone)]
enum ReadResponse {
    Read(weld::File),
    ListFiles(Vec<weld::File>),
}

impl<C: largetable_client::LargeTableClient, W: weld::WeldServer> Repo<C, W> {
    pub fn new(client: C) -> Self {
        let mut repo = Repo {
            db: client,
            remote_server: None,
            cache: Arc::new(Cache::new(CACHE_SIZE)),
            spaces: Arc::new(RwLock::new(HashMap::new())),
        };

        repo.initialize_space_map();

        repo
    }

    fn initialize_space_map(&mut self) {
        for change in self.list_changes().collect::<Vec<_>>() {
            self.spaces
                .write()
                .unwrap()
                .insert(change.get_friendly_name().to_owned(), change.get_id());
        }
    }

    pub fn lookup_friendly_name(&self, friendly_name: &str) -> Option<u64> {
        match self.spaces.read().unwrap().get(friendly_name) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    pub fn add_remote_server(&mut self, client: W) {
        self.remote_server = Some(client);
    }

    pub fn read_remote(&self, id: u64, path: &str, index: u64) -> Option<File> {
        let filename = normalize_filename(path);

        // First, check cache. If it's in there, quickly return.
        let query = ReadQuery::Read(id, path.to_owned(), index);

        match self.cache.get(&query) {
            Some(ReadResponse::Read(f)) => return if f.get_found() { Some(f) } else { None },
            _ => (),
        };

        match self.remote_server {
            Some(ref client) => {
                let mut ident = weld::FileIdentifier::new();
                ident.set_id(id);
                ident.set_filename(filename);
                ident.set_index(index);
                let file = client.read(ident);

                // Save to the cache, unless we're reading with index 0 (i.e. latest)
                if index != 0 {
                    self.cache.insert(query, ReadResponse::Read(file.clone()));
                }

                match file.get_found() {
                    true => Some(file),
                    false => None,
                }
            }
            // If we don't have a connected remote server, return nothing.
            None => None,
        }
    }

    pub fn read(&self, id: u64, path: &str, index: u64) -> Option<File> {
        let filename = normalize_filename(path);

        let change = match self.get_change(id) {
            Some(c) => c,
            None => return None,
        };

        // If the current change has a copy of the file, it must be the latest, so return it.
        if let Some(mut file) = self.db.read_proto::<File>(
            &change_to_rowname(id),
            path_to_colname(&filename).as_str(),
            index,
        ) {
            file.set_found(true);
            return Some(file);
        }

        // Otherwise we can fall back to the based change, if it exists.
        if change.get_is_based_locally() {
            self.read(change.get_based_id(), &filename, change.get_based_index())
        } else {
            self.read_remote(change.get_based_id(), &filename, change.get_based_index())
        }
    }

    pub fn write(&self, id: u64, mut file: File, index: u64) {
        // Create the associated parent directories.
        let mut directory = parent_directory(file.get_filename());
        while directory != "/" {
            self.create_directory(id, &directory, index);
            directory = parent_directory(&directory);
        }

        // Later, when the file is read, we should make sure we return
        // true for file.found.
        file.set_found(true);

        self.db.write_proto(
            change_to_rowname(id).as_str(),
            path_to_colname(file.get_filename()).as_str(),
            index,
            &file,
        );
    }

    pub fn delete(&self, id: u64, path: &str, index: u64) {
        let mut file = File::new();
        file.set_filename(path.to_owned());
        file.set_deleted(true);
        self.write(id, file, index)
    }

    pub fn create_directory(&self, id: u64, path: &str, index: u64) {
        // Check if the directory exists. If so, no work required.
        if self.read(id, path, index).is_some() {
            return;
        }

        let mut dir = File::new();
        dir.set_filename(path.to_owned());
        dir.set_directory(true);
        dir.set_found(true);

        self.db.write_proto(
            &change_to_rowname(id).as_str(),
            path_to_colname(path).as_str(),
            index,
            &dir,
        );
    }

    pub fn initialize_head(&mut self, id: u64) {
        self.db.write_proto(
            CHANGE_METADATA,
            &change_to_rowname(id),
            0,
            &weld::Change::new(),
        );
    }

    pub fn make_change(&self, mut change: weld::Change) -> u64 {
        // Reserve a local ID for this change.
        change.set_id(self.reserve_change_id());
        change.set_last_modified_timestamp(weld::get_timestamp_usec());

        // If based_space is empty and index default, and we are connected to a remote server, base
        // this on the remote server latest change ID.
        if !change.get_is_based_locally() && change.get_based_index() == 0 {
            if let Some(ref client) = self.remote_server {
                let latest_change = client.get_latest_change();
                change.set_based_id(0); // based on HEAD.
                change.set_based_index(latest_change.get_id());
            }
        }

        change.set_found(true);
        self.db.write_proto(
            CHANGE_METADATA,
            &change_to_rowname(change.get_id()),
            0,
            &change,
        );

        // Create an initial entry in the snapshots record.
        let mut entry = weld::SnapshotLogEntry::new();
        entry.set_is_rebase(true);
        entry.set_based_id(change.get_based_id());
        entry.set_based_index(change.get_based_index());
        self.log_snapshot(change.get_id(), entry);

        // Update the friendly name mapping.
        self.spaces
            .write()
            .unwrap()
            .insert(change.get_friendly_name().to_owned(), change.get_id());

        change.get_id()
    }

    fn log_snapshot(&self, id: u64, entry: weld::SnapshotLogEntry) {
        let row_name = format!("{}/{}", SNAPSHOTS, id);
        let snapshot_id = self.db.reserve_id(SNAPSHOT_IDS, &id.to_string());
        self.db
            .write_proto(&row_name, &snapshot_id.to_string(), 0, &entry);
    }

    pub fn get_change(&self, id: u64) -> Option<weld::Change> {
        self.db
            .read_proto(CHANGE_METADATA, &change_to_rowname(id), 0)
    }

    pub fn update_change(&self, change: &weld::Change) {
        self.spaces
            .write()
            .unwrap()
            .insert(change.get_friendly_name().to_owned(), change.get_id());

        self.db.write_proto(
            CHANGE_METADATA,
            &change_to_rowname(change.get_id()),
            0,
            change,
        );
    }

    pub fn list_changes(&self) -> impl Iterator<Item = weld::Change> + '_ {
        largetable_client::LargeTableScopedIterator::new(
            &self.db,
            String::from(CHANGE_METADATA),
            String::from(""),
            String::from(""),
            String::from(""),
            0,
        )
        .map(|(_, change)| change)
    }

    pub fn list_changed_files(&self, id: u64, index: u64) -> impl Iterator<Item = File> + '_ {
        largetable_client::LargeTableScopedIterator::new(
            &self.db,
            change_to_rowname(id),
            String::from(""),
            String::from(""),
            String::from(""),
            index,
        )
        .map(|(_, f)| f)
    }

    pub fn list_snapshots(&self, id: u64) -> impl Iterator<Item = weld::SnapshotLogEntry> + '_ {
        largetable_client::LargeTableScopedIterator::new(
            &self.db,
            format!("{}/{}", SNAPSHOTS, id),
            String::from(""),
            String::from(""),
            String::from(""),
            0,
        )
        .map(|(_, f)| f)
    }

    pub fn list_files_remote(&self, id: u64, directory: &str, index: u64) -> Vec<File> {
        // First, check cache. If it's in there, quickly return.
        let query = ReadQuery::ListFiles(id, directory.to_owned(), index);

        match self.cache.get(&query) {
            Some(ReadResponse::ListFiles(f)) => return f,
            _ => (),
        };

        match self.remote_server {
            Some(ref client) => {
                let mut ident = weld::FileIdentifier::new();
                ident.set_id(id);
                ident.set_filename(directory.to_owned());
                ident.set_index(index);
                let response = client.list_files(ident);

                // Save to the cache, unless we're reading with index 0 (i.e. latest)
                if index != 0 {
                    self.cache
                        .insert(query, ReadResponse::ListFiles(response.clone()));
                }

                response
            }
            None => vec![],
        }
    }

    pub fn list_files(&self, id: u64, directory: &str, index: u64) -> Vec<File> {
        // Need to make sure the last char in the string is a slash. Append one
        // if neccessary.
        let directory = normalize_directory(directory);

        let change = match self.get_change(id) {
            Some(c) => c,
            None => return vec![],
        };

        let mut files = std::collections::BTreeMap::new();
        for (_, file) in largetable_client::LargeTableScopedIterator::<File, _>::new(
            &self.db,
            change_to_rowname(id),
            path_to_colname(&directory),
            String::from(""),
            String::from(""),
            index,
        ) {
            files.insert(file.get_filename().to_owned(), file);
        }

        let based_files = if change.get_is_based_locally() {
            self.list_files(change.get_based_id(), &directory, change.get_based_index())
        } else {
            self.list_files_remote(change.get_based_id(), &directory, change.get_based_index())
        };

        for file in based_files {
            // Only insert if we don't already have a file for that filename.
            files.entry(file.get_filename().to_owned()).or_insert(file);
        }

        files
            .into_iter()
            .map(|(_, f)| f)
            .filter(|f| !f.get_deleted())
            .collect()
    }

    pub fn reserve_change_id(&self) -> u64 {
        self.db.reserve_id(CHANGE_IDS, "")
    }

    pub fn populate_change(&self, mut change: weld::Change) -> weld::Change {
        change.set_found(true);

        // First, get a list of all files touched by this change.
        // Then, go through all the snapshots. If there's a rebase, then insert
        // the version of the file at that moment.
        let snapshot_history = self.list_snapshots(change.get_id()).collect::<Vec<_>>();
        let mut files = HashMap::new();

        for snapshot in snapshot_history.iter() {
            for file in self.list_changed_files(change.get_id(), snapshot.get_index()) {
                let mut h = weld::FileHistory::new();
                h.set_filename(file.get_filename().to_owned());
                files.insert(file.get_filename().to_owned(), h);
            }
        }

        for (snapshot_id, snapshot) in snapshot_history.iter().enumerate() {
            // If the snapshot entry is a rebase, we need to pull all changed files
            // and enter the original version at this rebase.
            if snapshot.get_is_rebase() {
                for (_, history) in files.iter_mut() {
                    let mut ident = weld::FileIdentifier::new();
                    ident.set_id(snapshot.get_based_id());
                    ident.set_filename(history.get_filename().to_owned());
                    ident.set_index(snapshot.get_based_index());

                    let maybe_based_file = match change.get_is_based_locally() {
                        true => self.read(
                            snapshot.get_based_id(),
                            history.get_filename(),
                            snapshot.get_based_index(),
                        ),
                        false => self.read_remote(
                            snapshot.get_based_id(),
                            history.get_filename(),
                            snapshot.get_based_index(),
                        ),
                    };

                    if let Some(mut based_file) = maybe_based_file {
                        based_file.set_snapshot_id(snapshot_id as u64);
                        based_file.set_change_id(snapshot.get_based_index());
                        history.mut_snapshots().push(based_file);
                    }
                }

                continue;
            }
            // If it's not a rebase, that means we just need to include the changed
            // files in here.
            for mut file in self.list_changed_files(change.get_id(), snapshot.get_based_index()) {
                file.set_snapshot_id(snapshot_id as u64);
                file.set_change_id(0);
                let history = files.get_mut(file.get_filename()).unwrap();
                history.mut_snapshots().push(file);
            }
        }

        change.mut_changes().clear();
        for (_, history) in files.into_iter() {
            change.mut_changes().push(history);
        }

        change
    }

    pub fn snashot_from_id(&self, id: u64) -> weld::SnapshotResponse {
        let mut c = weld::Change::new();
        c.set_id(id);
        self.snapshot(&c)
    }

    // Fills a change proto with all staged file modifications
    pub fn fill_change(&self, change: &mut weld::Change) {
        let id = change.get_id();
        change.clear_staged_files();

        for file in self.list_changed_files(id, 0) {
            // Look up the remote file to figure out whether this file is identical to
            // the based version.
            let maybe_based_file = match change.get_is_based_locally() {
                true => self.read(
                    change.get_based_id(),
                    file.get_filename(),
                    change.get_based_index(),
                ),
                false => self.read_remote(
                    change.get_based_id(),
                    file.get_filename(),
                    change.get_based_index(),
                ),
            };

            let based_file = match maybe_based_file {
                Some(f) => f,
                None => weld::File::new(),
            };

            // If this file is a deletion, and he same file didn't exist in the remote repo,
            // then this is a no-op, and skip the file.
            if file.get_deleted() && !based_file.get_found() {
                continue;
            }

            // If the two protos are identical, then there's no change here, so ignore it.
            if file == based_file {
                continue;
            }

            change.mut_staged_files().push(file);
        }
    }

    // Create a patch for this change
    pub fn patch(&self, change: &weld::Change) -> String {
        let mut output = String::new();
        output += "From: Weld <weld@weld.io>\n";
        output += &format!(
            "Subject: [PATCH 1/1] {}\n\n",
            weld::summarize_change(change)
        );

        for file in change.get_staged_files() {
            // Look up the based file to create diff
            let maybe_based_file = match change.get_is_based_locally() {
                true => self.read(
                    change.get_based_id(),
                    file.get_filename(),
                    change.get_based_index(),
                ),
                false => self.read_remote(
                    change.get_based_id(),
                    file.get_filename(),
                    change.get_based_index(),
                ),
            };

            let based_file = match maybe_based_file {
                Some(f) => f,
                None => weld::File::new(),
            };

            // Temporarily write the files to disk
            let (filename_a, git_path_a) = if !file.get_deleted() {
                let filename = "/tmp/weld_a";
                let mut f = std::fs::File::create(filename).unwrap();
                f.write_all(file.get_contents());
                f.sync_data();
                (filename, format!("a{}", file.get_filename()))
            } else {
                ("/dev/null", String::from("/dev/null"))
            };

            let (filename_b, git_path_b) = if based_file.get_found() {
                let filename = "/tmp/weld_b";
                let mut f = std::fs::File::create(filename).unwrap();
                f.write_all(based_file.get_contents());
                f.sync_data();
                (filename, format!("b{}", based_file.get_filename()))
            } else {
                ("/dev/null", String::from("/dev/null"))
            };

            let diff = Command::new("diff")
                .args(&[
                    "--label",
                    &git_path_b,
                    "--label",
                    &git_path_a,
                    "-u",
                    filename_b,
                    filename_a,
                ])
                .output()
                .expect("Failed to create patch: is the `diff` command available?");

            output += std::str::from_utf8(&diff.stdout).unwrap();
        }

        output
    }

    pub fn snapshot(&self, partial_change: &weld::Change) -> weld::SnapshotResponse {
        let mut change = match self.get_change(partial_change.get_id()) {
            Some(c) => c,
            None => return weld::SnapshotResponse::new(),
        };

        // Use the fields from the partial change to update the change.
        weld::deserialize_change(&weld::serialize_change(partial_change, false), &mut change)
            .unwrap();

        self.update_change(&change);

        // Create an entry in the SNAPSHOTS record with the current filesystem state.
        let id = partial_change.get_id();
        let mut entry = weld::SnapshotLogEntry::new();
        let snapshot_id = weld::get_timestamp_usec();
        entry.set_index(snapshot_id);
        self.log_snapshot(id, entry);

        // Fill the change with all modified files
        self.fill_change(&mut change);

        // If we are basing against a remote souce, report the snapshot back to the remote source.
        if change.get_is_based_locally() || self.remote_server.is_none() {
            let mut response = weld::SnapshotResponse::new();
            response.set_change_id(change.get_id());
            response.set_snapshot_id(snapshot_id);

            return response;
        }

        // Since this is going to the remote server, we need to reframe the change into the remote
        // server's frame. That means converting the is_based_locally to true and setting the
        // remote_id to the real id.
        let mut remote_change = change.clone();
        remote_change.set_id(change.get_remote_id());
        remote_change.set_is_based_locally(true);

        let response = self.remote_server.as_ref().unwrap().snapshot(remote_change);

        // Potentially update the pending ID, if one was assigned.
        if change.get_remote_id() != response.get_change_id() {
            // Strip out the staged files since they might be a lot of data.
            change.mut_staged_files().clear();
            change.set_remote_id(response.get_change_id());
        }
        self.update_change(&change);

        response
    }

    pub fn submit(&self, id: u64) -> weld::SubmitResponse {
        let mut change = match self.get_change(id) {
            Some(c) => c,
            None => {
                println!("[repo] tried to submit unknown change {}", id);
                return weld::SubmitResponse::new();
            }
        };

        // Since this is going to the remote server, we need to reframe the change into the remote
        // server's frame. That means converting the is_based_locally to true and setting the
        // remote_id to the real id.
        let remote_id = change.get_remote_id();
        change.set_id(remote_id);
        change.set_is_based_locally(true);

        let response = self.remote_server.as_ref().unwrap().submit(change);

        if response.get_id() != 0 {
            // The submit was successful. Delete the client.
            self.delete_change(id);
        }

        response
    }

    pub fn delete_change(&self, id: u64) {
        let change = match self.get_change(id) {
            Some(c) => c,
            None => return eprintln!("Tried to delete non-existant change {}", id),
        };

        self.db.delete(CHANGE_METADATA, &change_to_rowname(id));
        self.spaces
            .write()
            .unwrap()
            .remove(change.get_friendly_name());
    }
}

pub fn parent_directory(filename: &str) -> String {
    // Remove the trailing slash, if it exists.
    let trimmed_filename = filename.trim_matches('/');

    let filename_parts: Vec<&str> = trimmed_filename.split('/').collect();
    let mut directory = String::from("/");
    for index in 0..filename_parts.len() - 1 {
        directory += filename_parts[index];
        if index != filename_parts.len() - 2 {
            directory += "/";
        }
    }

    directory
}

pub fn normalize_directory(directory: &str) -> String {
    format!("{}/", normalize_filename(directory).trim_right_matches('/'))
}

pub fn normalize_filename(filename: &str) -> String {
    format!("/{}", filename.trim_matches('/'))
}

fn change_to_rowname(id: u64) -> String {
    format!("{}/{}", CHANGES, id)
}

pub fn path_to_colname(path: &str) -> String {
    let depth = path.split("/").count();
    format!("{}:{}", depth, path)
}
