use std::fs::DirEntry;

use {colored::*, regex::Regex, zellij_tile::prelude::*};

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    selected_index: usize,

    directory_entries: Vec<DirEntry>,

    prompt: String,

    error: Option<Error>,
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Encountered an unnamed file.")]
    UnnamedFile,
}

static RIGHT_ARROW: &str = "▶";

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[EventType::TabUpdate, EventType::Key]);

        if let Err(error) = self.load_directory_entries() {
            self.error = Some(error);
        }
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        let mut should_sort = false;

        match event {
            Event::TabUpdate(tabs) => {
                self.tabs = tabs;
                should_render = true;
            }
            Event::Key(key) => match key {
                Key::Ctrl('n') | Key::Down => {
                    loop {
                        self.selected_index += 1;

                        if self.selected_index >= self.tabs.len() {
                            self.selected_index = 0;
                        }

                        if !self.tabs[self.selected_index].active {
                            break;
                        }
                    }

                    should_render = true;
                }
                Key::Ctrl('p') | Key::Up => {
                    loop {
                        if self.selected_index == 0 {
                            self.selected_index = self.tabs.len() - 1;
                        } else {
                            self.selected_index -= 1;
                        }

                        if !self.tabs[self.selected_index].active {
                            break;
                        }
                    }

                    should_render = true;
                }
                Key::Ctrl('c') => {
                    close_focus();
                }
                Key::Char('\n') => {
                    if let Err(error) = self.handle_selection() {
                        self.error = Some(error);
                        should_render = true;
                    }
                }
                Key::Char(input) => {
                    self.prompt.push(input);
                    self.selected_index = 0;

                    should_sort = true;
                    should_render = true;
                }
                Key::Backspace => {
                    self.prompt.pop();

                    should_sort = true;
                    should_render = true;
                }
                _ => {}
            },
            _ => {
                eprintln!("Got unrecognized event: {:?}", event);
            }
        }

        if should_sort {
            if let Err(error) = self.sort_directory_entries() {
                self.error = Some(error);
                should_render = true;
            }
        }

        should_render
    }

    fn render(&mut self, rows: usize, columns: usize) {
        if let Some(error) = &self.error {
            eprintln!("{}", error);
            print!("{}", error);

            return;
        }

        let body = self
            .directory_entries
            .iter()
            .take(rows - 4)
            .filter_map(|entry| {
                entry
                    .path()
                    .file_name()
                    .and_then(|osstr| osstr.to_str())
                    .map(String::from)
            })
            .enumerate()
            .map(|(index, name)| {
                let mut columns = columns;

                let arrow_or_space = if index == self.selected_index {
                    RIGHT_ARROW
                } else {
                    " "
                };

                columns -= arrow_or_space.len();

                columns -= name.len();

                let line = format!(" {} {}", arrow_or_space, name);

                if index == self.selected_index {
                    let rest_of_line = format!("{fill:width$}", fill = " ", width = columns);

                    format!("{}{}\n", line.reversed(), rest_of_line.reversed())
                } else {
                    format!("{}\n", line)
                }
            })
            .collect::<String>();

        let prompt = format!("\n > {}\n\n", self.prompt);

        print!("{}{}", prompt, body);
    }
}

impl State {
    fn create_tab(&self, name: &str) -> Result<(), Error> {
        let content = std::fs::read_to_string(
            "/host/zellij-tab-manager/config/layouts/directory/default.kdl",
        )?;

        let re = Regex::new(r"[$]name").unwrap();
        let tab_kdl = re.replace_all(&content, name);

        new_tabs_with_layout(&tab_kdl);

        Ok(())
    }

    fn load_directory_entries(&mut self) -> Result<(), Error> {
        self.directory_entries = vec![];

        let directory_entries = std::fs::read_dir("/host")?;

        for directory_entry in directory_entries.filter_map(Result::ok) {
            let metadata = directory_entry.metadata()?;

            if metadata.is_dir() {
                self.directory_entries.push(directory_entry);
            }
        }

        Ok(())
    }

    fn sort_directory_entries(&mut self) -> Result<(), Error> {
        use std::time::Instant;

        let start = Instant::now();

        self.directory_entries
            .sort_by_cached_key(|directory_entry| {
                if let Some(name) = directory_entry
                    .path()
                    .file_name()
                    .and_then(|name| name.to_str())
                {
                    SortScore(1.0 - strsim::jaro_winkler(&self.prompt, name))
                } else {
                    SortScore(1.0)
                }
            });

        eprintln!("sort duration: {:?}", start.elapsed());

        Ok(())
    }

    fn handle_selection(&self) -> Result<(), Error> {
        let directory_entry = &self.directory_entries[self.selected_index];
        let path_buf = directory_entry.path();

        let name = path_buf
            .file_name()
            .and_then(|name| name.to_str().to_owned())
            .ok_or(Error::UnnamedFile)?;

        let maybe_tab = self.tabs.iter().find(|tab| &tab.name == name);

        close_focus();

        if let Some(tab) = maybe_tab {
            switch_tab_to((tab.position + 1) as u32);
        } else {
            self.create_tab(name)?;
        }

        Ok(())
    }
}

#[derive(PartialEq, PartialOrd)]
struct SortScore(f64);

impl Eq for SortScore {}

impl Ord for SortScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
