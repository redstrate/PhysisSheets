#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BGMSwitch {
exd: EXD,
exh: EXH,
}
impl BGMSwitch {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BGMSwitch").unwrap();let exd = game_data.read_excel_sheet("BGMSwitch", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BGMSwitchRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BGMSwitchRow { columns: row.columns.clone() }
}
}
pub struct BGMSwitchRow {
columns: Vec<ColumnData>,
}
impl BGMSwitchRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BGM(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BGMSystemDefine(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
}
