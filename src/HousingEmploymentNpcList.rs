#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HousingEmploymentNpcList {
exd: EXD,
exh: EXH,
}
impl HousingEmploymentNpcList {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HousingEmploymentNpcList").unwrap();let exd = game_data.read_excel_sheet("HousingEmploymentNpcList", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HousingEmploymentNpcListRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HousingEmploymentNpcListRow { columns: row.columns.clone() }
}
}
pub struct HousingEmploymentNpcListRow {
columns: Vec<ColumnData>,
}
impl HousingEmploymentNpcListRow {
pub fn MaleENpcBase(&self) -> &ColumnData {
&self.columns[0]
}
pub fn FemaleENpcBase(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Race(&self) -> &ColumnData {
&self.columns[2]
}
}
