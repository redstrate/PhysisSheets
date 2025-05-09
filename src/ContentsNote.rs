#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentsNote {
exd: EXD,
exh: EXH,
}
impl ContentsNote {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentsNote").unwrap();let exd = game_data.read_excel_sheet("ContentsNote", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentsNoteRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentsNoteRow { columns: row.columns.clone() }
}
}
pub struct ContentsNoteRow {
columns: Vec<ColumnData>,
}
impl ContentsNoteRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ReqUnlock(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[3]
}
pub fn RequiredAmount(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ExpMultiplier(&self) -> &ColumnData {
&self.columns[5]
}
pub fn GilRward(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ExpCap(&self) -> &ColumnData {
&self.columns[7]
}
pub fn LevelUnlock(&self) -> &ColumnData {
&self.columns[8]
}
pub fn HowTo(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ContentType(&self) -> &ColumnData {
&self.columns[10]
}
pub fn MenuOrder(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Reward0(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Reward1(&self) -> &ColumnData {
&self.columns[13]
}
}
