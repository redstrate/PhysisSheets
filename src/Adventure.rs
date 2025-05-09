#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Adventure {
exd: EXD,
exh: EXH,
}
impl Adventure {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Adventure").unwrap();let exd = game_data.read_excel_sheet("Adventure", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> AdventureRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
AdventureRow { columns: row.columns.clone() }
}
}
pub struct AdventureRow {
columns: Vec<ColumnData>,
}
impl AdventureRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Impression(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Level(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MinLevel(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[5]
}
pub fn IconList(&self) -> &ColumnData {
&self.columns[6]
}
pub fn IconDiscovered(&self) -> &ColumnData {
&self.columns[7]
}
pub fn IconUndiscovered(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Emote(&self) -> &ColumnData {
&self.columns[9]
}
pub fn MinTime(&self) -> &ColumnData {
&self.columns[10]
}
pub fn MaxTime(&self) -> &ColumnData {
&self.columns[11]
}
pub fn MaxLevel(&self) -> &ColumnData {
&self.columns[12]
}
pub fn IsInitial(&self) -> &ColumnData {
&self.columns[13]
}
}
