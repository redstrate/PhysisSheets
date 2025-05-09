#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingAppeal {
exd: EXD,
exh: EXH,
}
impl HousingAppeal {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingAppeal").unwrap();let exd = game_data.read_excel_sheet("HousingAppeal", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingAppealRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingAppealRow { columns: row.columns.clone() }
}
}
pub struct HousingAppealRow {
columns: Vec<ColumnData>,
}
impl HousingAppealRow {
pub fn Tag(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[2]
}
}
