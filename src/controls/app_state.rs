use crate::util::NoDebug;
use crate::{
    on_refresh_select_all_modes_checkbox, on_refresh_set_bpm_or_delay_button, AppControls,
    FloatWrapper,
};
use libamx::{InitialTiming, LegacyMode, StxFile};
use statistical::mode as statistical_mode;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug)]
pub struct AppState {
    ptr: NoDebug<Rc<RefCell<AppControls>>>,
    stx_file: Option<StxFile>,
    cache_difficulty: HashMap<LegacyMode, i32>,
    cache_selection: HashSet<LegacyMode>,
    cache_stats: HashMap<LegacyMode, String>,
    cache_timing: HashMap<LegacyMode, InitialTiming>,
    next_delete: HashSet<LegacyMode>,
    next_export: Option<LegacyMode>,
    next_import: Option<LegacyMode>,
}

impl AppState {
    pub const PLACEHOLDER: &'static str = "∅";

    pub fn clear_cache(&mut self) {
        self.cache_difficulty.clear();
        self.cache_selection.clear();
        self.cache_stats.clear();
        self.cache_timing.clear();
        self.next_delete.clear();
        self.next_export = None;
        self.next_import = None;
    }

    pub fn close_file(&mut self) {
        self.stx_file = None;
        self.clear_cache();
    }

    pub fn get_bpm(&mut self, mode: LegacyMode) -> Option<String> {
        let initial_timing = self.get_initial_timing(mode).map(|timing| timing.bpm())?;
        Some(format!("{:.3}", initial_timing))
    }

    pub fn get_delay(&mut self, mode: LegacyMode) -> Option<String> {
        let initial_timing = self
            .get_initial_timing(mode)
            .map(|timing| timing.offset_ms())?;
        Some(format!("{:.0} ms", initial_timing))
    }

    pub fn get_difficulty(&mut self, mode: LegacyMode) -> Option<String> {
        if self.stx_file.is_none() {
            return None;
        }
        let difficulty = self.cache_difficulty.entry(mode).or_insert_with(|| {
            let stx_file = self.stx_file.as_ref().unwrap();
            stx_file
                .get_mode_info(mode)
                .map(|info| info.get_difficulty() as i32)
                .unwrap_or(i32::MIN)
        });
        if difficulty == &i32::MIN {
            return None;
        }

        Some(format!("{}", *difficulty))
    }

    pub fn get_initial_timing(&mut self, mode: LegacyMode) -> Option<InitialTiming> {
        if self.stx_file.is_none() {
            return None;
        }
        let initial_timing = self.cache_timing.entry(mode).or_insert_with(|| {
            let stx_file = self.stx_file.as_ref().unwrap();
            let initial_timings = stx_file.get_initial_timing(mode);

            let vec_bpm = initial_timings
                .iter()
                .map(|timing| FloatWrapper(timing.bpm()))
                .collect::<Vec<FloatWrapper<f64>>>();
            let vec_offset_ms = initial_timings
                .iter()
                .map(|timing| FloatWrapper(timing.offset_ms()))
                .collect::<Vec<FloatWrapper<f64>>>();

            match (statistical_mode(&vec_bpm), statistical_mode(&vec_offset_ms)) {
                (Some(bpm), Some(offset_ms)) => {
                    InitialTiming::new(bpm.into_inner(), offset_ms.into_inner())
                }
                (Some(bpm), None) => {
                    InitialTiming::new(bpm.into_inner(), InitialTiming::default().offset_ms())
                }
                (None, Some(offset_ms)) => {
                    InitialTiming::new(InitialTiming::default().bpm(), offset_ms.into_inner())
                }
                (None, None) => InitialTiming::default(),
            }
        });

        if initial_timing.bpm().is_nan() && initial_timing.offset_ms().is_nan() {
            None
        } else {
            Some(InitialTiming::new(
                initial_timing.bpm(),
                initial_timing.offset_ms(),
            ))
        }
    }

    pub fn get_is_selected(&self, mode: LegacyMode) -> Option<i32> {
        if self.stx_file.is_none() {
            return None;
        }
        Some(self.cache_selection.contains(&mode) as i32)
    }

    pub fn get_is_selected_all(&self) -> Option<bool> {
        if self.stx_file.is_none() {
            return None;
        }
        Some(self.cache_selection.len() == LegacyMode::COUNT)
    }

    pub fn get_is_selected_none(&self) -> Option<bool> {
        if self.stx_file.is_none() {
            return None;
        }
        Some(self.cache_selection.is_empty())
    }

    pub fn get_stats(&mut self, mode: LegacyMode) -> Option<String> {
        if self.stx_file.is_none() {
            return None;
        }
        let stats = self.cache_stats.entry(mode).or_insert_with(|| {
            let stx_file = self.stx_file.as_ref().unwrap();
            stx_file
                .get_mode_info(mode)
                .map(|info| (info.get_num_splits(), info.count_blocks()))
                .map(|(num_splits, num_blocks)| format!("{}s / {}b", num_splits, num_blocks))
                .unwrap_or(Self::PLACEHOLDER.to_string())
        });
        if stats == Self::PLACEHOLDER {
            return None;
        }

        Some(stats.clone())
    }

    pub fn get_step_file(&self) -> Option<&StxFile> {
        self.stx_file.as_ref()
    }

    pub fn get_step_file_mut(&mut self) -> Option<&mut StxFile> {
        self.stx_file.as_mut()
    }

    pub fn is_enabled(&self) -> bool {
        self.stx_file.is_some()
    }

    pub fn modal_err(&self, title: &str, description: &str) {
        self.ptr
            .0
            .borrow()
            .get_main_win()
            .modal_err(title, description);
    }

    pub fn new(ptr: Rc<RefCell<AppControls>>) -> Self {
        AppState {
            ptr: NoDebug(ptr),
            stx_file: None,
            cache_difficulty: HashMap::default(),
            cache_selection: HashSet::default(),
            cache_stats: HashMap::default(),
            cache_timing: HashMap::default(),
            next_delete: HashSet::default(),
            next_export: None,
            next_import: None,
        }
    }

    pub fn open_file(&mut self, file: StxFile) {
        self.stx_file = Some(file);
        self.clear_cache();
    }

    pub fn set_select_all(&mut self) {
        if self.stx_file.is_none() {
            return;
        }
        for mode in LegacyMode::iter() {
            self.cache_selection.insert(mode);
        }
    }

    pub fn set_select_none(&mut self) {
        if self.stx_file.is_none() {
            return;
        }
        self.cache_selection.clear();
    }

    pub fn set_next_delete(&mut self, mode: LegacyMode) {
        if self.stx_file.is_none() {
            return;
        }
        if self.next_delete.contains(&mode) {
            self.next_delete.remove(&mode);
        } else {
            self.next_delete.insert(mode);
        }
    }

    pub fn set_next_export(&mut self, mode: Option<LegacyMode>) {
        if self.stx_file.is_none() {
            return;
        }
        self.next_export = mode;
    }

    pub fn set_next_import(&mut self, mode: Option<LegacyMode>) {
        if self.stx_file.is_none() {
            return;
        }
        self.next_import = mode;
    }

    pub fn toggle_is_selected(&mut self, mode: LegacyMode) {
        if self.stx_file.is_none() {
            return;
        }
        if self.cache_selection.contains(&mode) {
            self.cache_selection.remove(&mode);
        } else {
            self.cache_selection.insert(mode);
        }
        on_refresh_select_all_modes_checkbox(
            self.ptr.0.clone(),
            self.stx_file.is_some(),
            self.cache_selection.len() == LegacyMode::COUNT,
        );
        on_refresh_set_bpm_or_delay_button(
            self.ptr.0.clone(),
            self.stx_file.is_some(),
            self.cache_selection.is_empty(),
        )
    }
}
