#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct McGuffinUIData {
exd: EXD,
exh: EXH,
}
impl McGuffinUIData {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("McGuffinUIData").unwrap();let exd = game_data.read_excel_sheet("McGuffinUIData", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> McGuffinUIDataRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
McGuffinUIDataRow { columns: row.columns.clone() }
}
}
pub struct McGuffinUIDataRow {
columns: Vec<ColumnData>,
}
impl McGuffinUIDataRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[2]
}
}
