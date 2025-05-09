#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PartyContent {
exd: EXD,
exh: EXH,
}
impl PartyContent {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PartyContent").unwrap();let exd = game_data.read_excel_sheet("PartyContent", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PartyContentRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PartyContentRow { columns: row.columns.clone() }
}
}
pub struct PartyContentRow {
columns: Vec<ColumnData>,
}
impl PartyContentRow {
pub fn LGBEventObject(&self) -> &ColumnData {
&self.columns[0]
}
pub fn LGBEventRange(&self) -> &ColumnData {
&self.columns[1]
}
pub fn LGBEventObject2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn TextDataStart(&self) -> &ColumnData {
&self.columns[3]
}
pub fn TextDataEnd(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[5]
}
pub fn TimeLimit(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ContentFinderCondition(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Key(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[11]
}
}
