#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DawnQuestMember {
exd: EXD,
exh: EXH,
}
impl DawnQuestMember {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DawnQuestMember").unwrap();let exd = game_data.read_excel_sheet("DawnQuestMember", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DawnQuestMemberRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DawnQuestMemberRow { columns: row.columns.clone() }
}
}
pub struct DawnQuestMemberRow {
columns: Vec<ColumnData>,
}
impl DawnQuestMemberRow {
pub fn Member(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BigImageOld(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BigImageNew(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Class(&self) -> &ColumnData {
&self.columns[5]
}
}
