#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MonsterNoteTarget {
exd: EXD,
exh: EXH,
}
impl MonsterNoteTarget {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MonsterNoteTarget").unwrap();let exd = game_data.read_excel_sheet("MonsterNoteTarget", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MonsterNoteTargetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MonsterNoteTargetRow { columns: row.columns.clone() }
}
}
pub struct MonsterNoteTargetRow {
columns: Vec<ColumnData>,
}
impl MonsterNoteTargetRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BNpcName(&self) -> &ColumnData {
&self.columns[1]
}
pub fn PlaceNameZone(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PlaceNameLocation(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Town(&self) -> &ColumnData {
&self.columns[4]
}
}
