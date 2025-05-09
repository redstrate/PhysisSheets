#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct NotebookDivision {
exd: EXD,
exh: EXH,
}
impl NotebookDivision {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("NotebookDivision").unwrap();let exd = game_data.read_excel_sheet("NotebookDivision", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> NotebookDivisionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
NotebookDivisionRow { columns: row.columns.clone() }
}
}
pub struct NotebookDivisionRow {
columns: Vec<ColumnData>,
}
impl NotebookDivisionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn QuestUnlock(&self) -> &ColumnData {
&self.columns[1]
}
pub fn NotebookDivisionCategory(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CraftOpeningLevel(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GatheringOpeningLevel(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CRPCraft(&self) -> &ColumnData {
&self.columns[6]
}
pub fn BSMCraft(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ARMCraft(&self) -> &ColumnData {
&self.columns[8]
}
pub fn GSMCraft(&self) -> &ColumnData {
&self.columns[9]
}
pub fn LTWCraft(&self) -> &ColumnData {
&self.columns[10]
}
pub fn WVRCraft(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ALCCraft(&self) -> &ColumnData {
&self.columns[12]
}
pub fn CULCraft(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[14]
}
}
