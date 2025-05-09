#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct RelicNote {
exd: EXD,
exh: EXH,
}
impl RelicNote {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("RelicNote").unwrap();let exd = game_data.read_excel_sheet("RelicNote", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RelicNoteRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
RelicNoteRow { columns: row.columns.clone() }
}
}
pub struct RelicNoteRow {
columns: Vec<ColumnData>,
}
impl RelicNoteRow {
pub fn EventItem(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MonsterNoteTargetCommon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn MonsterNoteTargetNM(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Fate(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PlaceNameFate(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Leve(&self) -> &ColumnData {
&self.columns[6]
}
pub fn MonsterCount(&self) -> &ColumnData {
&self.columns[7]
}
}
