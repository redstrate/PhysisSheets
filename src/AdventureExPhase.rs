#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct AdventureExPhase {
exd: EXD,
exh: EXH,
}
impl AdventureExPhase {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("AdventureExPhase").unwrap();let exd = game_data.read_excel_sheet("AdventureExPhase", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AdventureExPhaseRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AdventureExPhaseRow { columns: row.columns.clone() }
}
}
pub struct AdventureExPhaseRow {
columns: Vec<ColumnData>,
}
impl AdventureExPhaseRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn AdventureBegin(&self) -> &ColumnData {
&self.columns[1]
}
pub fn AdventureEnd(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Expansion(&self) -> &ColumnData {
&self.columns[4]
}
}
