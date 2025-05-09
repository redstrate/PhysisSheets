#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MountTransient {
exd: EXD,
exh: EXH,
}
impl MountTransient {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MountTransient").unwrap();let exd = game_data.read_excel_sheet("MountTransient", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MountTransientRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MountTransientRow { columns: row.columns.clone() }
}
}
pub struct MountTransientRow {
columns: Vec<ColumnData>,
}
impl MountTransientRow {
pub fn Description(&self) -> &ColumnData {
&self.columns[0]
}
pub fn DescriptionEnhanced(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Tooltip(&self) -> &ColumnData {
&self.columns[2]
}
}
