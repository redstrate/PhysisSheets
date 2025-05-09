#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct WeaponTimeline {
exd: EXD,
exh: EXH,
}
impl WeaponTimeline {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("WeaponTimeline").unwrap();let exd = game_data.read_excel_sheet("WeaponTimeline", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> WeaponTimelineRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
WeaponTimelineRow { columns: row.columns.clone() }
}
}
pub struct WeaponTimelineRow {
columns: Vec<ColumnData>,
}
impl WeaponTimelineRow {
pub fn File(&self) -> &ColumnData {
&self.columns[0]
}
pub fn NextWeaponTimeline(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
}
