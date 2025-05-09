#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CreditList {
exd: EXD,
exh: EXH,
}
impl CreditList {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CreditList").unwrap();let exd = game_data.read_excel_sheet("CreditList", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CreditListRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CreditListRow { columns: row.columns.clone() }
}
}
pub struct CreditListRow {
columns: Vec<ColumnData>,
}
impl CreditListRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Font(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Cast(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Scale(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
}
