#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SharlayanCraftWorks {
exd: EXD,
exh: EXH,
}
impl SharlayanCraftWorks {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SharlayanCraftWorks").unwrap();let exd = game_data.read_excel_sheet("SharlayanCraftWorks", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SharlayanCraftWorksRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SharlayanCraftWorksRow { columns: row.columns.clone() }
}
}
pub struct SharlayanCraftWorksRow {
columns: Vec<ColumnData>,
}
impl SharlayanCraftWorksRow {
pub fn Description(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Questgiver(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[2]
}
}
